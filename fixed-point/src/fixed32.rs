//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use typenum::Unsigned;
use std::{
	ops, fmt,
  convert::TryInto,
	marker::PhantomData,
};

///	A 32 bit fixed point number.
/// 
/// The `Shift` parameter determines how many of the lower bits are used for the
/// fractional components.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash,)]
pub struct Fixed32<Shift: Unsigned,>(i32, PhantomData<Shift>,);

impl<Shift: Unsigned,> Fixed32<Shift,> {
  const I32SHIFT: i32 = 1 << Shift::I32;
  const I64SHIFT: i64 = Self::I32SHIFT as i64;
  const F32SHIFT: f32 = Self::I32SHIFT as f32;

  /// Converts this value to an `i32`.
  #[inline]
  pub const fn to_i32(self,) -> i32 { self.0 / Self::I32SHIFT }
  /// Converts this value to an `f32`.
  #[inline]
  pub const fn to_f32(self,) -> f32 { self.0 as f32 / Self::F32SHIFT }
}

impl<Shift: Unsigned,> From<i32,> for Fixed32<Shift,> {
  #[inline]
  fn from(from: i32,) -> Self { Fixed32(from * Self::I32SHIFT, PhantomData,) }
}

impl<Shift: Unsigned,> Into<i32,> for Fixed32<Shift,> {
  #[inline]
  fn into(self,) -> i32 { self.to_i32() }
}

impl<Shift: Unsigned,> From<f32,> for Fixed32<Shift,> {
  #[inline]
  fn from(from: f32,) -> Self { Fixed32(
    (from * Self::F32SHIFT).round() as i32,
    PhantomData,
  ) }
}

impl<Shift: Unsigned,> Into<f32,> for Fixed32<Shift,> {
  #[inline]
  fn into(self,) -> f32 { self.to_f32() }
}

impl<Shift: Unsigned,> Clone for Fixed32<Shift,> {
  #[inline]
  fn clone(&self,) -> Self { Fixed32(self.0, PhantomData,) }
}

impl<Shift: Unsigned,> Copy for Fixed32<Shift,> {}

impl<Shift: Unsigned,> ops::Neg for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn neg(mut self,) -> Self::Output { self.0 = -self.0; self }
}

impl<Shift: Unsigned,> ops::Add for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn add(mut self, rhs: Self,) -> Self::Output { self += rhs; self }
}

impl<Shift: Unsigned,> ops::AddAssign for Fixed32<Shift,> {
  #[inline]
  fn add_assign(&mut self, rhs: Self,) { self.0 += rhs.0 }
}

impl<Shift: Unsigned,> ops::Sub for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn sub(mut self, rhs: Self,) -> Self::Output { self -= rhs; self }
}

impl<Shift: Unsigned,> ops::SubAssign for Fixed32<Shift,> {
  #[inline]
  fn sub_assign(&mut self, rhs: Self,) { self.0 -= rhs.0 }
}

impl<Shift: Unsigned,> ops::Mul for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: Self,) -> Self::Output { self *= rhs; self }
}

impl<Shift: Unsigned,> ops::MulAssign for Fixed32<Shift,> {
  #[inline]
  fn mul_assign(&mut self, rhs: Self,) {
    self.0 = (self.0 as i64 * rhs.0 as i64 / Self::I64SHIFT).try_into()
      .expect("Fixed32 Multiplication overflowed");
  }
}

impl<Shift: Unsigned,> ops::Mul<i32> for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: i32,) -> Self::Output { self *= rhs; self }
}

impl<Shift: Unsigned,> ops::MulAssign<i32> for Fixed32<Shift,> {
  #[inline]
  fn mul_assign(&mut self, rhs: i32,) { self.0 *= rhs }
}

impl<Shift: Unsigned,> ops::Div for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: Self,) -> Self::Output { self /= rhs; self }
}

impl<Shift: Unsigned,> ops::DivAssign for Fixed32<Shift,> {
  #[inline]
  fn div_assign(&mut self, rhs: Self,) {
    self.0 = (self.0 as i64 * Self::I64SHIFT / rhs.0 as i64).try_into()
      .expect("Fixed32 Division overflowed")
  }
}

impl<Shift: Unsigned,> ops::Div<i32> for Fixed32<Shift,> {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: i32,) -> Self::Output { self /= rhs; self }
}

impl<Shift: Unsigned,> ops::DivAssign<i32> for Fixed32<Shift,> {
  #[inline]
  fn div_assign(&mut self, rhs: i32,) { self.0 /= rhs }
}

impl<Shift: Unsigned,> fmt::Debug for Fixed32<Shift,> {
  #[inline]
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    fmt.debug_tuple("Fixed32",)
    .field(&self.to_f32(),)
    .finish()
  }
}

#[cfg(test,)]
mod tests {
  use super::*;
  use typenum::U16;

  #[test]
  fn test_fixed32() {
    assert_eq!(10, Fixed32::<U16>::from(10).to_i32(), "Convertion i32 failed",);
    assert!((10.8 - Fixed32::<U16>::from(10.8).to_f32()).powi(2) < 0.000001, "Convertion f32 failed",);

    let num = Fixed32::<U16>::from(1) + Fixed32::<U16>::from(2);
    assert_eq!(num, Fixed32::<U16>::from(3), "Addition failed",);

    let num = Fixed32::<U16>::from(1) - Fixed32::<U16>::from(2);
    assert_eq!(num, Fixed32::<U16>::from(-1), "Subtraction failed",);

    let num = Fixed32::<U16>::from(2) * Fixed32::<U16>::from(3);
    assert_eq!(num, Fixed32::<U16>::from(6), "Multiplication failed",);

    let num = Fixed32::<U16>::from(2) * 3;
    assert_eq!(num, Fixed32::<U16>::from(6), "Multiplication i32 failed",);

    let num = Fixed32::<U16>::from(1) / Fixed32::<U16>::from(2);
    assert_eq!(num.to_f32(), 0.5, "Division failed",);

    let num = Fixed32::<U16>::from(1) / 2;
    assert_eq!(num.to_f32(), 0.5, "Division i32 failed",);
  }
}
