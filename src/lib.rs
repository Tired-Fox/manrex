mod error;


mod client;
pub mod model;

pub use error::Error;
pub use client::{Client, auth};
