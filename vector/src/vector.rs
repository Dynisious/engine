//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use crate::*;
use std::{ops, cmp::Ordering,};

/// A 3D Vector.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash,)]
pub struct Vector<Num,> {
  /// The coordinate in the x dimention.
  pub x: Num,
  /// The coordinate in the y dimention.
  pub y: Num,
  /// The coordinate in the z dimention.
  pub z: Num,
}

impl<Num,> Vector<Num,> {
  /// Builds a new Vector value.
  #[inline]
  pub const fn new(x: Num, y: Num, z: Num,) -> Self { Self { x, y, z } }
}

impl<Num: Number,> Vector<Num,> {
  /// Returns the dot product of two Vectors.
  #[inline]
  pub fn dot(lhs: Self, rhs: Self,) -> Num { lhs * rhs }
}

impl<Num,> Vector<Num,>
  where Num: Number + Sqrt + Trigonometry + Clone + From<f32>, {
  /// Rotates this Vector.
  /// 
  /// # Params
  /// 
  /// rotation --- The rotation to apply.  
  pub fn rotate(&self, rotation: &Rotation<Num,>,) -> Self {
    /*
    Quaternion Identities:
    ijk = -1
    ij = k, ji = -k
    jk = i, kj = -i
    ki = j, ik = -j

    t = q.u
      = (a + bi + cj + dk)(xi + yj + zk)
      = i(ax + cz - dy) + j(ay + dx - bz) + k(ad + by - cx) - bx - cy - dz
      = w + i(ax + cz - dy) + j(ay + dx - bz) + k(ad + by - cx)
    v = t.q-1
      = (w + xi + yj + zk)(a + bi + cj + dk)
      = i(wb + ax + yd - zc) + j(wc + ay + zb - xd) + k(wd + az + xc - yb) + wa - xb - yc - zd

    x dim is i
    y dim is j
    z dim is k
    */
    let angle = rotation.angle.clone() / 2.0.into();
    let a = angle.clone().sin();
    let mut bcd = rotation.axis.clone().vector() * angle.cos();
    let temp = Vector::new(
      (a.clone() * self.x.clone()) + (bcd.y.clone() * self.z.clone())
        - (bcd.z.clone() * self.y.clone()),
      (a.clone() * self.y.clone()) + (bcd.z.clone() * self.x.clone())
        - (bcd.x.clone() * self.z.clone()),
      (a.clone() * self.z.clone()) + (bcd.x.clone() * self.y.clone())
        - (bcd.y.clone() * self.x.clone()),
    );
    let w = -(bcd.x.clone() * self.x.clone())
      - (bcd.y.clone() * self.y.clone())
      - (bcd.z.clone() * self.z.clone());

    //Negate as the inversion of the quaternion.
    bcd = -bcd;
    
    Vector::new(
      (w.clone() * bcd.x.clone()) + (a.clone() * temp.x.clone())
        + (temp.y.clone() * bcd.z.clone()) - (temp.z.clone() * bcd.y.clone()),
      (w.clone() * bcd.y.clone()) + (a.clone() * temp.y.clone())
        + (temp.z.clone() * bcd.x.clone()) - (temp.x.clone() * bcd.z.clone()),
      (w * bcd.z) + (a * temp.z) + (temp.x * bcd.y) - (temp.y * bcd.x),
    )
  }
}

impl<Num,> Vector<Num,>
  where Num: Clone + ops::Mul<Output = Num> + ops::Sub<Output = Num>, {
  /// Returns the dot product of two Vectors.
  pub fn cross(lhs: Self, rhs: Self,) -> Self {
    let x = (lhs.y.clone() * rhs.z.clone()) - (lhs.z.clone() * rhs.y.clone());
    let y = (lhs.x.clone() * rhs.z) - (lhs.z * rhs.x.clone());
    let z = (lhs.x * rhs.y) - (lhs.y * rhs.x);

    Self { x, y, z, }
  }
}

impl<Num,> Vector<Num,>
  where Num: Sqrt + Clone, {
  /// Returns the magnituid of this Vector.
  #[inline]
  pub fn magnituid(self,) -> Num { Self::dot(self.clone(), self,).sqrt() }
  /// Converts this Vector into a unit Vector.
  #[inline]
  pub fn unit(self,) -> Unit<Num,> { self.into() }
}

impl<Num: Default,> Vector<Num,> {
  /// Returns a Vector populated with the default value of `Num`.
  pub fn origin() -> Self { Self::new(
    Num::default(),
    Num::default(),
    Num::default(),
  ) }
}

impl<Num,> From<(Num, Num, Num,)> for Vector<Num,> {
  #[inline]
  fn from((x, y, z,): (Num, Num, Num,),) -> Self { Self { x, y, z, } }
}

impl<Num: PartialOrd,> PartialOrd for Vector<Num,> {
  fn partial_cmp(&self, rhs: &Self,) -> Option<Ordering> {
    self.x.partial_cmp(&rhs.x,)
    .and_then(|lhs,| self.y.partial_cmp(&rhs.y,).filter(move |rhs,| lhs == *rhs,),)
    .and_then(|lhs,| self.z.partial_cmp(&rhs.z,).filter(move |rhs,| lhs == *rhs,),)
  }
}

impl<Num,> ops::Neg for Vector<Num,>
  where Num: Clone + ops::Neg<Output = Num>, {
  type Output = Self;
  
  fn neg(mut self,) -> Self {
    self.x = -self.x.clone();
    self.y = -self.y.clone();
    self.z = -self.z.clone();

    self
  }
}

impl<Num,> ops::Add for Vector<Num,>
  where Num: Clone + ops::Add<Output = Num>, {
  type Output = Self;

  #[inline]
  fn add(mut self, rhs: Self,) -> Self::Output { self += rhs; self }
}

impl<Num,> ops::AddAssign for Vector<Num,>
  where Num: Clone + ops::Add<Output = Num>, {
  #[inline]
  fn add_assign(&mut self, rhs: Self,) {
    self.x = self.x.clone() + rhs.x;
    self.y = self.y.clone() + rhs.y;
    self.z = self.z.clone() + rhs.z;
  }
}

impl<Num,> ops::Sub for Vector<Num,>
  where Num: Clone + ops::Sub<Output = Num>, {
  type Output = Self;

  #[inline]
  fn sub(mut self, rhs: Self,) -> Self::Output { self -= rhs; self }
}

impl<Num,> ops::SubAssign for Vector<Num,>
  where Num: Clone + ops::Sub<Output = Num>, {
  fn sub_assign(&mut self, rhs: Self,) {
    self.x = self.x.clone() - rhs.x;
    self.y = self.y.clone() - rhs.y;
    self.z = self.z.clone() - rhs.z;
  }
}

impl<Num,> ops::Mul<Num> for Vector<Num,>
  where Num: Clone + ops::Mul<Output = Num>, {
  type Output = Self;

  #[inline]
  fn mul(mut self, rhs: Num,) -> Self::Output { self *= rhs; self }
}

impl<Num,> ops::MulAssign<Num> for Vector<Num,>
  where Num: Clone + ops::Mul<Output = Num>, {
  fn mul_assign(&mut self, rhs: Num,) {
    self.x = self.x.clone() * rhs.clone();
    self.y = self.y.clone() * rhs.clone();
    self.z = self.z.clone() * rhs;
  }
}

impl<Num,> ops::Div<Num> for Vector<Num,>
  where Num: Clone + ops::Div<Output = Num>, {
  type Output = Self;

  #[inline]
  fn div(mut self, rhs: Num,) -> Self::Output { self /= rhs; self }
}

impl<Num: Clone + ops::Div<Output = Num>,> ops::DivAssign<Num> for Vector<Num,> {
  fn div_assign(&mut self, rhs: Num,) {
    self.x = self.x.clone() / rhs.clone();
    self.y = self.y.clone() / rhs.clone();
    self.z = self.z.clone() / rhs;
  }
}

impl<Num> ops::Mul for Vector<Num,>
  where Num: ops::Mul<Output = Num> + ops::Add<Output = Num>, {
  type Output = Num;

  fn mul(self, rhs: Self,) -> Self::Output {
    (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
  }
}

/// A [Vector] with a length of 1 at all times.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash,)]
pub struct Unit<Num>(Vector<Num,>,);

impl<Num> Unit<Num,> {
  /// Converts this [Unit] into a [Vector].
  #[inline]
  pub fn vector(self,) -> Vector<Num,> { self.0 }
}

impl<Num: Sqrt + Clone,> From<Vector<Num,>> for Unit<Num,> {
  #[inline]
  fn from(from: Vector<Num,>,) -> Self { Unit(from.clone() / from.magnituid(),) }
}

impl<Num,> Into<Vector<Num,>> for Unit<Num,> {
  #[inline]
  fn into(self,) -> Vector<Num,> { self.0 }
}

impl<Num: PartialEq,> PartialEq<Vector<Num,>> for Unit<Num,> {
  #[inline]
  fn eq(&self, rhs: &Vector<Num,>,) -> bool { self.0 == *rhs }
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
    assert_eq!(vec * vec, Vector::dot(vec, vec,), "Dot product function failed",);

    let x = (1.0, 0.0, 0.0,).into();
    let y = (0.0, 1.0, 0.0,).into();
    let z = (0.0, 0.0, 1.0,).into();

    assert_eq!(Vector::cross(x, y,), z, "Cross product failed 1",);
    assert_eq!(Vector::cross(y, z,), x, "Cross product failed 2",);
    assert_eq!(Vector::cross(x, z,), y, "Cross product failed 3",);

    const PI2: f32 = std::f32::consts::FRAC_PI_2;
    const EPSILON: f32 = std::f32::EPSILON;

    let rot = Rotation::new(z.into(), PI2,);
    let vec = x.rotate(&rot,);
    let diff = 1.0 - vec.y;
    assert!(0.0 < diff && diff < EPSILON, "Rotate failed 1: {:?}", vec,);

    let rot = Rotation::new(x.into(), PI2,);
    let vec = y.rotate(&rot,);
    let diff = 1.0 - vec.z;
    assert!(0.0 < diff && diff < EPSILON, "Rotate failed 2: {:?}", vec,);

    let rot = Rotation::new(y.into(), PI2,);
    let vec = z.rotate(&rot,);
    let diff = 1.0 - vec.x;
    assert!(0.0 < diff && diff < EPSILON, "Rotate failed 3: {:?}", vec,);
  }
}
