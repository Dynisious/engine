//! Defines traits for transformation of types.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use vector::{Vector, Unit, Rotation, Number, Sqrt,};
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

impl<Num: Number + Clone,> Position for Vector<Num,> {
  type Pos = Self;

  #[inline]
  fn position(&self,) -> Self::Pos { self.clone() }
  #[inline]
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self { *self = pos; self }
}

/// A trait for types with an orientation.
pub trait Orientation<Num: Number + Clone,> {
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
  fn rotate(&mut self, rotation: &Rotation,) -> &mut Self {
    self.set_direction(Vector::rotate(&self.direction(), rotation,),)
  }
}

/// Extends [Orientation] behaviour.
pub trait OrientationExt<Num: Sqrt + Clone,>: Orientation<Num,> {
  /// Returns the [Unit] of this objects direction.
  #[inline]
  fn orientation(&self,) -> Unit<Num,> { self.direction().unit() }
}

impl<Num, T,> OrientationExt<Num,> for T
  where Num: Sqrt + Clone, T: Orientation<Num,>, {}

impl<Num: Number + Clone,> Orientation<Num,> for Vector<Num,> {
  #[inline]
  fn direction(&self,) -> Vector<Num,> { self.clone() }
  #[inline]
  fn set_direction(&mut self, dir: Self,) -> &mut Self { *self = dir; self }
}
