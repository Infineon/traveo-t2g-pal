#![no_std]

#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[allow(unused_imports)]
use crate::generic::*;
#[cfg(cm4)]
mod m4;
#[cfg(cm4)]
pub use self::m4::*;
#[cfg(cm0)]
mod m0;
#[cfg(cm0)]
pub use self::m0::*;
