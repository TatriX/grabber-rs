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
mod tests {}
