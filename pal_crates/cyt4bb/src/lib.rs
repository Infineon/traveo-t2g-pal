#![no_std]

#[cfg(all(feature = "rev_a"))]
pub use cyt4bb_a::*;
#[cfg(all(feature = "rev_b"))]
pub use cyt4bb_b::*;
