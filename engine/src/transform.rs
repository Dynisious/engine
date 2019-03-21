//! Defines traits for transformation of types.
//! 
//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-7

use vector::{Vector, Number, Sqrt,};
use std::ops::Add;

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
    where Self::Pos: Add<Trans, Output = Self::Pos>, Trans: Clone, {
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

#[derive(Clone, Copy,)]
pub struct Rotation<Num: Number> {
  pub axis: Vector<Num,>,
  pub angle: f32,
}

pub trait Orientation<Num: Number,> {
  fn direction(&self,) -> Vector<Num,>;
  fn set_direction(&mut self, dir: Vector<Num,>,) -> &mut Self;
  #[inline]
  fn rotate<Rot,>(&mut self, rotation: Rotation<Rot,>,) -> &mut Self
    where Rot: Number, {
    self.set_direction(self.direction().rotate(rotation,),)
  }
}

pub trait OrientationExt<Num: Sqrt,> {
  #[inline]
  fn orientation(&self,) -> Vector<Num,> { self.direction().unit() }
}

impl<Num: Number + Clone,> Orientation<Num,> for Vector<Num,> {
  #[inline]
  fn direction(&self,) -> Vector<Num,> { self.clone() }
  #[inline]
  fn set_direction(&mut self, dir: Self,) -> &mut Self { *self = dir; self }
}
