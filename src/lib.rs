//! [cli] -(config, urls)-> [core]
//! config: [Download, Parse, Format]
//! Download(url) -> RawData
//! Parse(RawData) -> Entity
//! Format(Entity) -> Write
#![feature(conservative_impl_trait)]

// TODO: use cpu_pool

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

// compiler warns about unused import when it's actually used
#[allow(unused)]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate currency;
extern crate num_traits;
extern crate scraper;

#[cfg(test)]
extern crate mockito;

pub mod download;
pub mod parse;

#[cfg(test)]
mod tests {
    // $("#productTitle").textContent
    // $$(".offer-price")[0].textContent
    // $("#imgBlkFront").src

    use mockito::{self, mock};
    use hyper::Uri;
    use tokio_core::reactor::Core;
    use download::http::Https;
    use download::Download;
    use futures::Future;
    use parse::Parse;
    use parse::xml::Xml;
    use currency::Currency;
    use num_traits::cast::ToPrimitive;

    #[test]
    fn http_json() {
        let body = r#"
            <html>
             <body>
              <div id="title">Title</div>
              <img id="img" src="image.png">
              <span class="price">$9.99</span>
             </body>
            </html>
        "#;

        #[derive(Debug, PartialEq, Serialize)]
        struct Product {
            title: String,
            image: String,
            price: f64,
        }

        let _m = mock("GET", "/").with_body(body).create();

        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let uri: Uri = mockito::SERVER_URL.parse().unwrap();

        let task = Https::new(&handle).download(uri).map(|buf| {
            let parser = Xml::new(|document| {
                Ok(Product {
                    title: document.select("#title").text().unwrap(),
                    image: document.select("#img").attr("src").unwrap(),
                    price: Currency::from_str(&document.select(".price").text().unwrap())
                        .unwrap()
                        .value()
                        .to_f64()
                        .unwrap() / 100.0,
                })
            });
            let product = parser.parse(buf).unwrap();

            assert_eq!(product, {
                Product {
                    title: "Title".to_string(),
                    image: "image.png".to_string(),
                    price: 9.99,
                }
            });
        });

        core.run(task).unwrap();
    }
}
