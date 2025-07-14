//! 2-dimensional grids.

#![no_std]

pub mod buf;

pub mod core;

pub(crate) mod internal;

mod error;
pub use error::GridError;
