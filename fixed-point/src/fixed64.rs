//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-07

use typenum::Unsigned;
use std::{
	ops, fmt,
  convert::TryInto,
	marker::PhantomData,
};

///	A 64 bit fixed point number.
/// 
/// The `Shift` parameter determines how many of the lower bits are used for the
/// fractional components.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash,)]
pub struct Fixed64<Shift: Unsigned,>(i64, PhantomData<Shift>,);

impl<Shift: Unsigned,> Fixed64<Shift,> {
  const I64SHIFT: i64 = 1 << Shift::I64;
  const I128SHIFT: i128 = Self::I64SHIFT as i128;
  const F32SHIFT: f64 = Self::I64SHIFT as f64;

  /// Converts this value to an `i64`.
  #[inline]
  pub const fn to_i64(self,) -> i64 { self.0 / Self::I64SHIFT }
  /// Converts this value to an `f64`.
  #[inline]
  pub const fn to_f64(self,) -> f64 { self.0 as f64 / Self::F32SHIFT }
}

impl<Shift: Unsigned,> From<i64,> for Fixed64<Shift,> {
  #[inline]
  fn from(from: i64,) -> Self { Fixed64(from * Self::I64SHIFT, PhantomData,) }
}

impl<Shift: Unsigned,> Into<i64,> for Fixed64<Shift,> {
  #[inline]
  fn into(self,) -> i64 { self.to_i64() }
}

impl<Shift: Unsigned,> From<f64,> for Fixed64<Shift,> {
  #[inline]
  fn from(from: f64,) -> Self { Fixed64(
    (from * Self::F32SHIFT).round() as i64,
    PhantomData,
  ) }
}

impl<Shift: Unsigned,> Into<f64,> for Fixed64<Shift,> {
  #[inline]
  fn into(self,) -> f64 { self.to_f64() }
}

impl<Shift: Unsigned,> Clone for Fixed64<Shift,> {
  #[inline]
  fn clone(&self,) -> Self { Fixed64(self.0, PhantomData,) }
}

impl<Shift: Unsigned,> Copy for Fixed64<Shift,> {}

impl<Shift: Unsigned,> ops::Add for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn add(mut self, rhs: Self,) -> Self::Output { self += rhs; self }
}

impl<Shift: Unsigned,> ops::AddAssign for Fixed64<Shift,> {
  #[inline]
  fn add_assign(&mut self, rhs: Self,) { self.0 += rhs.0 }
}

impl<Shift: Unsigned,> ops::Sub for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn sub(mut self, rhs: Self,) -> Self::Output { self -= rhs; self }
}

impl<Shift: Unsigned,> ops::SubAssign for Fixed64<Shift,> {
  #[inline]
  fn sub_assign(&mut self, rhs: Self,) { self.0 -= rhs.0 }
}

impl<Shift: Unsigned,> ops::Mul for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: Self,) -> Self::Output { self *= rhs; self }
}

impl<Shift: Unsigned,> ops::MulAssign for Fixed64<Shift,> {
  #[inline]
  fn mul_assign(&mut self, rhs: Self,) {
    self.0 = (self.0 as i128 * rhs.0 as i128 / Self::I128SHIFT).try_into()
      .expect("Fixed64 Multiplication overflowed");
  }
}

impl<Shift: Unsigned,> ops::Mul<i64> for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: i64,) -> Self::Output { self *= rhs; self }
}

impl<Shift: Unsigned,> ops::MulAssign<i64> for Fixed64<Shift,> {
  #[inline]
  fn mul_assign(&mut self, rhs: i64,) { self.0 *= rhs }
}

impl<Shift: Unsigned,> ops::Div for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: Self,) -> Self::Output { self /= rhs; self }
}

impl<Shift: Unsigned,> ops::DivAssign for Fixed64<Shift,> {
  #[inline]
  fn div_assign(&mut self, rhs: Self,) {
    self.0 = (self.0 as i128 * Self::I128SHIFT / rhs.0 as i128).try_into()
      .expect("Fixed64 Division overflowed")
  }
}

impl<Shift: Unsigned,> ops::Div<i64> for Fixed64<Shift,> {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: i64,) -> Self::Output { self /= rhs; self }
}

impl<Shift: Unsigned,> ops::DivAssign<i64> for Fixed64<Shift,> {
  #[inline]
  fn div_assign(&mut self, rhs: i64,) { self.0 /= rhs }
}

impl<Shift: Unsigned,> fmt::Debug for Fixed64<Shift,> {
  #[inline]
  fn fmt(&self, fmt: &mut fmt::Formatter,) -> fmt::Result {
    fmt.debug_tuple("Fixed64",)
    .field(&self.to_f64(),)
    .finish()
  }
}

#[cfg(test,)]
mod tests {
  use super::*;
  use typenum::U16;

  #[test]
  fn test_fixed64() {
    assert_eq!(10, Fixed64::<U16>::from(10).to_i64(), "Convertion i64 failed",);
    assert!((10.8 - Fixed64::<U16>::from(10.8).to_f64()).powi(2) < 0.000001, "Convertion f64 failed",);

    let num = Fixed64::<U16>::from(1) + Fixed64::<U16>::from(2);
    assert_eq!(num, Fixed64::<U16>::from(3), "Addition failed",);

    let num = Fixed64::<U16>::from(1) - Fixed64::<U16>::from(2);
    assert_eq!(num, Fixed64::<U16>::from(-1), "Subtraction failed",);

    let num = Fixed64::<U16>::from(2) * Fixed64::<U16>::from(3);
    assert_eq!(num, Fixed64::<U16>::from(6), "Multiplication failed",);

    let num = Fixed64::<U16>::from(2) * 3;
    assert_eq!(num, Fixed64::<U16>::from(6), "Multiplication i64 failed",);

    let num = Fixed64::<U16>::from(1) / Fixed64::<U16>::from(2);
    assert_eq!(num.to_f64(), 0.5, "Division failed",);

    let num = Fixed64::<U16>::from(1) / 2;
    assert_eq!(num.to_f64(), 0.5, "Division i64 failed",);
  }
}
