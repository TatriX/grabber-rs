use std::io;
use futures::Future;

pub mod http;

pub trait Download {
    type Url;
    type Item: io::Read + ?Sized;
    fn download(self, url: Self::Url) -> Box<Future<Item = Self::Item, Error = io::Error>>;
}
