#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(test), no_std)]
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod async_api;
pub mod control;
pub mod datetime;
mod ds1307;
pub mod error;
pub mod nvram;
pub mod registers;
pub mod square_wave;

pub use ds1307::Ds1307;
pub use rtc_hal::datetime::DateTime;
