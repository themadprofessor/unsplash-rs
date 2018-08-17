#![feature(option_replace)]
#![feature(external_doc)]
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![doc(include = "../README.md")]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate itertools;
extern crate serde;
extern crate serde_json;
extern crate serde_url_params;

/// Root URI of the Unsplash API.
pub const API_URL: &'static str = "https://api.unsplash.com/";

/// Endpoints of the Unsplash API.
pub mod endpoint;

/// Errors that can be raised.
pub mod error;

pub use endpoint::{me::Me, photos::Photos};
