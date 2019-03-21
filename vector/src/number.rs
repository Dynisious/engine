//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

#[cfg(feature = "fixed-point",)]
use fixed_point::Unsigned;
use std::ops;

/// Defines a type for numbers.
pub trait Number: ops::Add<Output = Self>
  + ops::Sub<Output = Self>
  + ops::Mul<Output = Self>
  + ops::Div<Output = Self>
  + ops::Neg<Output = Self>
  + Sized {
  /// Creates a new number from a `isize`.
  fn from_isize(from: isize,) -> Self;
}

impl Number for isize {
  #[inline]
  fn from_isize(from: isize,) -> Self { from }
}

impl Number for i8 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as i8 }
}

impl Number for i16 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as i16 }
}

impl Number for i32 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as i32 }
}

impl Number for i64 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as i64 }
}

impl Number for i128 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as i128 }
}

impl Number for f32 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as f32 }
}

impl Number for f64 {
  #[inline]
  fn from_isize(from: isize,) -> Self { from as f64 }
}

/// Included with feature "fixed-point".
#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Number for fixed_point::Fixed32<Shift,> {
  #[inline]
  fn from_isize(from: isize,) -> Self { (from as i32).into() }
}

/// Included with feature "fixed-point".
#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Number for fixed_point::Fixed64<Shift,> {
  #[inline]
  fn from_isize(from: isize,) -> Self { (from as i64).into() }
}

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

/// Included with feature "fixed-point".
#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Sqrt for fixed_point::Fixed32<Shift,> {
  fn sqrt(self,) -> Self { self.to_f32().sqrt().into() }
}

/// Included with feature "fixed-point".
#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Sqrt for fixed_point::Fixed64<Shift,> {
  fn sqrt(self,) -> Self { self.to_f64().sqrt().into() }
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
}

#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Trigonometry for fixed_point::Fixed32<Shift,> {
  fn sin(self,) -> Self { f32::sin(self.to_f32(),).into() }
  fn cos(self,) -> Self { f32::cos(self.to_f32(),).into() }
  fn tan(self,) -> Self { f32::tan(self.to_f32(),).into() }
  fn asin(self,) -> Self { f32::asin(self.to_f32(),).into() }
  fn acos(self,) -> Self { f32::acos(self.to_f32(),).into() }
  fn atan(self,) -> Self { f32::atan(self.to_f32(),).into() }
}

#[cfg(feature = "fixed-point",)]
impl<Shift: Unsigned,> Trigonometry for fixed_point::Fixed64<Shift,> {
  fn sin(self,) -> Self { f64::sin(self.to_f64(),).into() }
  fn cos(self,) -> Self { f64::cos(self.to_f64(),).into() }
  fn tan(self,) -> Self { f64::tan(self.to_f64(),).into() }
  fn asin(self,) -> Self { f64::asin(self.to_f64(),).into() }
  fn acos(self,) -> Self { f64::acos(self.to_f64(),).into() }
  fn atan(self,) -> Self { f64::atan(self.to_f64(),).into() }
}
