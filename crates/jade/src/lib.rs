#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub use {codec, jade_derive::*};

pub mod prelude;

#[cfg(not(feature = "std"))]
extern crate alloc;
