//! An implementation of 3D vectors.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

#![deny(missing_docs,)]
#![feature(const_fn,)]

use std::ops;

mod number;

pub use self::number::*;

/// A 3D Vector.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash,)]
pub struct Vector<Num: Number,> {
  /// The coordinate in the x dimention.
  pub x: Num,
  /// The coordinate in the y dimention.
  pub y: Num,
  /// The coordinate in the z dimention.
  pub z: Num,
}

impl<Num: Number,> Vector<Num,> {
  /// Builds a new Vector value.
  #[inline]
  pub const fn new(x: Num, y: Num, z: Num,) -> Self { Self { x, y, z } }
}

impl<Num: Sqrt + Clone,> Vector<Num,> {
  /// Converts this Vector into a unit Vector.
  #[inline]
  pub fn unit(self,) -> Self { self.clone() / self.magnituid() }
  /// Returns the magnituid of this Vector.
  #[inline]
  pub fn magnituid(self,) -> Num { Self::dot(self.clone(), self,).sqrt() }
  /// Returns the dot product of two Vectors.
  #[inline]
  pub fn dot<Rhs,>(lhs: Self, rhs: Vector<Rhs,>,) -> Num
    where Rhs: Number, Num: ops::Mul<Rhs, Output = Num>, {
    (lhs.x * rhs.x) + (lhs.y * rhs.y) + (lhs.z * rhs.z)
  }
}

impl<Num: Number + Default,> Vector<Num,> {
  /// Returns a Vector populated with the default value of `Num`.
  pub fn origin() -> Self { Self::new(
    Num::default(),
    Num::default(),
    Num::default(),
  ) }
}

impl<Num: Number,> From<(Num, Num, Num,)> for Vector<Num,> {
  #[inline]
  fn from((x, y, z,): (Num, Num, Num,),) -> Self { Self { x, y, z, } }
}

impl<Num: Number + Clone,> ops::Add for Vector<Num,> {
  type Output = Self;

  #[inline]
  fn add(mut self, rhs: Self,) -> Self::Output { self += rhs; self }
}

impl<Num: Number + Clone,> ops::AddAssign for Vector<Num,> {
  #[inline]
  fn add_assign(&mut self, rhs: Self,) {
    self.x = self.x.clone() + rhs.x;
    self.y = self.y.clone() + rhs.y;
    self.z = self.z.clone() + rhs.z;
  }
}

impl<Num: Number + Clone,> ops::Sub for Vector<Num,> {
  type Output = Self;

  #[inline]
  fn sub(mut self, rhs: Self,) -> Self::Output { self -= rhs; self }
}

impl<Num: Number + Clone,> ops::SubAssign for Vector<Num,> {
  #[inline]
  fn sub_assign(&mut self, rhs: Self,) {
    self.x = self.x.clone() - rhs.x;
    self.y = self.y.clone() - rhs.y;
    self.z = self.z.clone() - rhs.z;
  }
}

impl<Num: Number + Clone,> ops::Mul<Num> for Vector<Num,> {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: Num,) -> Self::Output { self *= rhs; self }
}

impl<Num: Number + Clone,> ops::MulAssign<Num> for Vector<Num,> {
  #[inline]
  fn mul_assign(&mut self, rhs: Num,) {
    self.x = self.x.clone() * rhs.clone();
    self.y = self.y.clone() * rhs.clone();
    self.z = self.z.clone() * rhs;
  }
}

impl<Num: Number + Clone,> ops::Div<Num> for Vector<Num,> {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: Num,) -> Self::Output { self /= rhs; self }
}

impl<Num: Number + Clone,> ops::DivAssign<Num> for Vector<Num,> {
  #[inline]
  fn div_assign(&mut self, rhs: Num,) {
    self.x = self.x.clone() / rhs.clone();
    self.y = self.y.clone() / rhs.clone();
    self.z = self.z.clone() / rhs;
  }
}

impl<Num: Number,> ops::Mul for Vector<Num,> {
  type Output = Num;

  #[inline]
  fn mul(self, rhs: Self,) -> Self::Output {
    (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
  }
}

#[cfg(test,)]
mod tests {
  use super::*;

  #[test]
  fn test_vector() {
    let vec = Vector::new(1, 2, 3,);
    let vec2 = vec * 2;

    assert_eq!(vec + vec, Vector::new(2, 4, 6,), "Addition failed",);
    assert_eq!(vec2 - vec, vec, "Subtraction failed",);
    assert_eq!(vec + vec, vec2, "Multiplication failed",);
    assert_eq!(vec, vec2 / 2, "Division failed",);
    assert_eq!(vec * vec, 14, "Dot product failed",);
  }
}
