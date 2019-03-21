//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use std::ops;

/// Defines a type for numbers.
pub trait Number: ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + Sized {}

impl<T,> Number for T
  where T: ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + Sized {}

/// Defines a square root operation for a [Number] type.
pub trait Sqrt: Number {
  /// Returns the square root of this [Number].
  fn sqrt(self,) -> Self;
}

impl Sqrt for f32 {
  #[inline]
  fn sqrt(self,) -> Self { f32::sqrt(self,) }
}

impl Sqrt for f64 {
  #[inline]
  fn sqrt(self,) -> Self { f64::sqrt(self,) }
}
