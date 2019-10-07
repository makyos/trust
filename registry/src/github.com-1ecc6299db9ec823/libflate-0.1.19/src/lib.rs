//! A Rust implementation of DEFLATE algorithm and related formats (ZLIB, GZIP).
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]
extern crate adler32;
extern crate byteorder;
extern crate crc32fast;

pub use finish::Finish;

macro_rules! invalid_data_error {
    ($fmt:expr) => { invalid_data_error!("{}", $fmt) };
    ($fmt:expr, $($arg:tt)*) => {
        ::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!($fmt, $($arg)*))
    }
}

macro_rules! finish_try {
    ($e:expr) => {
        match $e.unwrap() {
            (inner, None) => inner,
            (inner, error) => return ::finish::Finish::new(inner, error),
        }
    };
}

pub mod deflate;
pub mod finish;
pub mod gzip;
pub mod lz77;
pub mod non_blocking;
pub mod zlib;

mod bit;
mod checksum;
mod huffman;
mod util;
