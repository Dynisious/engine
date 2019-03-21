//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use std::ops;

/// Defines a type for numbers.
pub trait Number: ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Neg<Output = Self>
  + Sized {}

impl<T,> Number for T
  where T: ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Neg<Output = Self>
  + Sized {}

/// Defines a square root operation for a number type.
pub trait Sqrt: Number {
  /// Returns the square root of this number.
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

/// Defines trigonometry operations for a number type.
pub trait Trigonometry: Number {
  /// The sine of this number.
  fn sin(self,) -> Self;
  /// The cosine of this number.
  fn cos(self,) -> Self;
  /// The tangent of this number.
  fn tan(self,) -> Self;
  /// The inverse sine of this number.
  fn asin(self,) -> Self;
  /// The inverse cosine of this number.
  fn acos(self,) -> Self;
  /// The inverse tangent of this number.
  fn atan(self,) -> Self;
  /// Creates a new number from a `usize`.
  fn from_usize(from: usize,) -> Self;
}

impl Trigonometry for f32 {
  #[inline]
  fn sin(self,) -> Self { f32::sin(self,) }
  #[inline]
  fn cos(self,) -> Self { f32::cos(self,) }
  #[inline]
  fn tan(self,) -> Self { f32::tan(self,) }
  #[inline]
  fn asin(self,) -> Self { f32::asin(self,) }
  #[inline]
  fn acos(self,) -> Self { f32::acos(self,) }
  #[inline]
  fn atan(self,) -> Self { f32::atan(self,) }
  #[inline]
  fn from_usize(from: usize,) -> Self { from as f32 }
}

impl Trigonometry for f64 {
  #[inline]
  fn sin(self,) -> Self { f64::sin(self,) }
  #[inline]
  fn cos(self,) -> Self { f64::cos(self,) }
  #[inline]
  fn tan(self,) -> Self { f64::tan(self,) }
  #[inline]
  fn asin(self,) -> Self { f64::asin(self,) }
  #[inline]
  fn acos(self,) -> Self { f64::acos(self,) }
  #[inline]
  fn atan(self,) -> Self { f64::atan(self,) }
  #[inline]
  fn from_usize(from: usize,) -> Self { from as f64 }
}
