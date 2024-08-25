#![no_std]
#![feature(try_trait_v2)]
extern crate alloc;

pub mod browser;
mod error;
pub mod os;
mod traits;
pub use error::*;
