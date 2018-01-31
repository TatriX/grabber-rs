//! [cli] -(config, urls)-> [core]
//! config: [Download, Parse, Format]
//! Download(url) -> RawData
//! Parse(RawData) -> Entity
//! Format(Entity) -> Write
#![feature(conservative_impl_trait)]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[cfg(test)]
extern crate mockito;
extern crate tokio_core;
// TODO: use cpu_pool

use std::io::{self, Cursor};
use futures::{Future, Stream};
use tokio_core::reactor::Handle;
use hyper::Client;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

pub trait Download {
    type Url;
    type Item: io::Read + ?Sized;
    fn download(self, url: Self::Url) -> Box<Future<Item = Self::Item, Error = io::Error>>;
}

pub struct DownloadHttp {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl DownloadHttp {
    pub fn new(handle: &Handle) -> DownloadHttp {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        DownloadHttp { client }
    }
}

impl Download for DownloadHttp {
    type Url = hyper::Uri;
    type Item = Cursor<Vec<u8>>;
    fn download(self, url: Self::Url) -> Box<Future<Item = Self::Item, Error = io::Error>> {
        Box::new(
            self.client
                .get(url)
                .and_then(|res| {
                    res.body()
                        .concat2()
                        .map(|chunk| Cursor::new(Vec::from(chunk.as_ref())))
                })
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{Future, Stream};
    use hyper::Client;
    use tokio_core::reactor::Core;
    use mockito::{self, mock};
    use hyper_tls::HttpsConnector;
    const URL: &'static str = mockito::SERVER_URL;

    #[test]
    fn core() {
        let body = "This is mockito!\n";
        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("Content-Type", "text/plain")
            .with_body(body)
            .create();

        let url = URL.parse().unwrap();

        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        let work = client.get(url).and_then(|res| {
            res.body().concat2().map(|chunk| {
                assert_eq!(chunk.as_ref(), body.as_bytes());
            })
        });
        core.run(work).unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn download() {
        let body = "Some random text";
        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("Content-Type", "text/plain")
            .with_body(body)
            .create();

        let uri: hyper::Uri = URL.parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let download = DownloadHttp::new(&handle).download(uri).map(|buf| {
            assert_eq!(String::from_utf8(buf.into_inner()).unwrap(), body);
        });
        core.run(download).unwrap();
    }

    #[test]
    fn parse() {}
}
