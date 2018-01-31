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

pub mod download;

#[cfg(test)]
mod tests {}
