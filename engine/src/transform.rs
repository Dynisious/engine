//! Defines traits for transformation of types.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-22

use vector::{Vector, Unit, Rotation, Sqrt, Trigonometry,};
use std::ops;

/// A trait for types with a position attribute.
pub trait Position {
  /// The type of the position attribute.
  type Pos;

  /// Returns the position of the value.
  fn position(&self,) -> Self::Pos;
  /// Updates the position of the value.
  /// 
  /// # Params
  /// 
  /// pos --- The new position value.  
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self;
  /// Translates the position of the value by the passed translation.
  /// 
  /// # Params
  /// 
  /// trans --- The translation to the position.  
  #[inline]
  fn translate<Trans,>(&mut self, trans: Trans,) -> &mut Self
    where Self::Pos: ops::Add<Trans, Output = Self::Pos>, Trans: Clone, {
    self.set_position(self.position() + trans,)
  }
}

impl<Num: Clone,> Position for Vector<Num,> {
  type Pos = Self;

  #[inline]
  fn position(&self,) -> Self::Pos { self.clone() }
  #[inline]
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self { *self = pos; self }
}

/// A trait for types with an orientation.
pub trait Orientation<Num,>
  where Num: Sqrt + Trigonometry + Clone, {
  /// Gets the direction this object is facing.
  fn direction(&self,) -> Vector<Num,>;
  /// Sets the direction this object is facing.
  fn set_direction(&mut self, dir: Vector<Num,>,) -> &mut Self;
  /// Rotates the direction this object is facing by the passed [Rotation].
  /// 
  /// # Params
  /// 
  /// rotation --- The rotation to perform.  
  #[inline]
  fn rotate(&mut self, rotation: &Rotation<Num,>,) -> &mut Self {
    self.set_direction(Vector::rotate(&self.direction(), rotation,),)
  }
}

/// Extends [Orientation] behaviour.
pub trait OrientationExt<Num,>: Orientation<Num,>
  where Num: Sqrt + Trigonometry + Clone, {
  /// Returns the [Unit] of this objects direction.
  fn orientation(&self,) -> Unit<Num,> { self.direction().unit() }
}

impl<Num, T,> OrientationExt<Num,> for T
  where T: Orientation<Num,>, Num: Sqrt + Trigonometry + Clone, {}

impl<Num: Clone,> Orientation<Num,> for Vector<Num,>
  where Num: Sqrt + Trigonometry + Clone, {
  #[inline]
  fn direction(&self,) -> Vector<Num,> { self.clone() }
  #[inline]
  fn set_direction(&mut self, dir: Self,) -> &mut Self { *self = dir; self }
}

#[cfg(test,)]
mod tests {
  use super::*;

  #[test]
  fn test_position() {
    let vec1 = Vector::new(1, 2, 3,);
    let vec2 = Vector::new(-3, -2, -1,);

    assert_eq!(vec1.position(), vec1, "position failed",);
    assert_eq!(*vec1.clone().translate(vec2,), (-2, 0, 2,).into(), "translate failed",);
    assert_eq!(*vec1.clone().set_position(vec2,), vec2, "set_position failed",);
  }
  #[test]
  fn test_orientaion() {
    let vec1 = Vector::new(1.0, 2.0, 3.0,);
    let vec2 = Vector::new(-3.0, -2.0, -1.0,);

    assert_eq!(vec1.direction(), vec1, "direction failed",);
    assert_eq!(*vec1.clone().set_direction(vec2,), vec2, "set_direction failed",);
    assert_eq!(vec1.clone().orientation(), vec1 / vec1.magnituid(), "orientation failed",);
  }
}
