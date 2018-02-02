use std::io;
use serde::Serialize;

pub mod xml;

pub trait Parse<T, E, A, F>
where
    T: Serialize,
    F: FnOnce(A) -> Result<T, E> + 'static,
{
    fn new(convert: F) -> Self;
    fn parse<S: io::Read>(self, buf: S) -> Result<T, E>;
}
