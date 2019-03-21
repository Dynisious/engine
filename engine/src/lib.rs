//! A small rendering engine.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-7

#![deny(missing_docs,)]

pub mod transform;
mod line;
mod triangle;

pub use self::line::*;
pub use self::triangle::*;
