#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod fmt;

mod protocol;
pub use protocol::*;

pub mod device;

pub mod service;

mod traits;
pub use traits::*;

mod updater;
pub use updater::*;
