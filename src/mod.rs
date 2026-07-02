#![cfg_attr(not(feature = "std"), no_std)]

// - types
#[cfg(feature = "std")]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// - mod
#[cfg(feature = "compression")]
pub mod compression;
pub mod constants;
#[cfg(feature = "read_at")]
pub mod read_at;
pub mod traits;

// - external
#[cfg(feature = "log")]
use log::{trace};