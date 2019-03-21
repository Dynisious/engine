//!	Defines types for fixed point arithmetic.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-07

#![deny(missing_docs,)]
#![feature(const_fn,)]

pub use typenum::{consts, Unsigned, UInt, UTerm, bit::{B0, B1,},};

mod fixed32;
mod fixed64;

pub use self::fixed32::*;
pub use self::fixed64::*;
