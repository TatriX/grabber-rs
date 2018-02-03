extern crate currency;
extern crate futures;
extern crate grabber;
extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

// TODO: add grabber::prelude::*

use tokio_core::reactor::{Core, Handle};
use grabber::download::http::Https;
use grabber::download::Download;
use grabber::parse::xml::Xml;
use grabber::parse::Parse;
use currency::Currency;
use num_traits::cast::ToPrimitive;
use futures::future::{self, Future};
use std::collections::HashMap;
use std::io;

#[derive(Debug, PartialEq, Serialize)]
struct Product {
    title: String,
    image: String,
    price: f64,
}

fn grab(handle: &Handle, url: &str) -> Box<Future<Item = Product, Error = io::Error>> {
    let uri = url.parse().unwrap();

    let task = Https::new(&handle).download(uri).map(|buf| {
        let parser = Xml::new(|document| {
            let images = document
                .select("#imgBlkFront")
                .attr("data-a-dynamic-image")
                .unwrap();
            let map: HashMap<String, [i32; 2]> = serde_json::from_str(&images).unwrap();

            // order is undefined so search for the biggest image
            let (image, _) = map.into_iter().max_by_key(|&(_, size)| size[0]).unwrap();

            Ok(Product {
                title: document.select("#productTitle").text().unwrap(),
                image,
                price: Currency::from_str(&document.select(".offer-price").text().unwrap())
                    .unwrap()
                    .value()
                    .to_f64()
                    .unwrap() / 100.0,
            })
        });
        parser.parse(buf).unwrap()
    });
    Box::new(task)
}

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let urls = vec![
        "https://www.amazon.co.uk/gp/product/1509836071",
        "https://www.amazon.co.uk/gp/product/0008239339",
        "https://www.amazon.co.uk/gp/product/1780723040",
        "https://www.amazon.co.uk/gp/product/071818887X",
        "https://www.amazon.co.uk/gp/product/0091901650",
    ];

    let tasks = urls.iter().map(|url| {
        grab(&handle, url).inspect(move |product| println!("{}\n{:#?}\n", url, product))
    });
    core.run(future::join_all(tasks)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grab_one() {
        let mut core = Core::new().unwrap();
        let task = grab(
            &core.handle(),
            "https://www.amazon.co.uk/gp/product/1509836071",
        ).map(|product| {
            assert_eq!(product, {
                Product {
                    title: "The Fat-Loss Plan: 100 Quick and Easy Recipes with Workouts".to_string(),
                    image: "https://images-na.ssl-images-amazon.com/images/I/51UvEYZVpbL._SX382_BO1,204,203,200_.jpg".to_string(),
                    price: 6.99,
                }
            });
        });

        core.run(task).unwrap();
    }
}
