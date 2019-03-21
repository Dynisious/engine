//! An implementation of 3D vectors.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

#![deny(missing_docs,)]
#![feature(const_fn,)]

mod number;
mod vector;
mod rotation;

pub use self::{number::*, vector::*, rotation::*,};
