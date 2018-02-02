use std::io::{self, Cursor};
use futures::{Future, Stream};
use tokio_core::reactor::Handle;
use hyper::{self, Client};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use download::Download;

pub struct Https {
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Https {
    pub fn new(handle: &Handle) -> Https {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &handle).unwrap())
            .build(&handle);

        Https { client }
    }
}

impl Download for Https {
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
    use futures::Future;
    use hyper::Uri;
    use tokio_core::reactor::Core;
    use mockito::{self, mock};
    use download::Download;
    use download::http::Https;

    #[test]
    fn download() {
        let body = "En Taro Adun";
        let _m = mock("GET", "/").with_body(body).create();

        let uri: Uri = mockito::SERVER_URL.parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let download = Https::new(&handle).download(uri).map(|buf| {
            assert_eq!(String::from_utf8(buf.into_inner()).unwrap(), body);
        });
        core.run(download).unwrap();
    }
}
