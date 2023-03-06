#![no_std]

#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[allow(unused_imports)]
use crate::generic::*;
#[cfg(cm7)]
mod m7;
#[cfg(cm7)]
pub use self::m7::*;
#[cfg(cm0)]
mod m0;
#[cfg(cm0)]
pub use self::m0::*;
