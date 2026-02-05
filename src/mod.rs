#![cfg_attr(not(feature = "std"), no_std)]

// - types
#[cfg(feature = "std")]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// - mod
#[cfg(feature = "compression")]
pub mod compression;

// - external
#[cfg(feature = "log")]
use log::{trace};