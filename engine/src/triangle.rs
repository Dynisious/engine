//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use crate::transform::{Position, Orientation,};
use vector::{Number, Sqrt, Vector, Rotation,};
use std::ops;

/// A triangle of three points in space.
#[derive(PartialEq, Eq, Clone, Copy, Hash,)]
pub struct Triangle<Num: Number,> {
  /// The first point.
  pub p1: Vector<Num,>,
  /// The second point.
  pub p2: Vector<Num,>,
  /// The third point.
  pub p3: Vector<Num,>,
}

impl<Num: Number + Clone + From<usize>,> Position for Triangle<Num,> {
  type Pos = Vector<Num,>;

  fn position(&self,) -> Self::Pos {
    (self.p1.clone() + self.p2.clone() + self.p3.clone()) / 3.into()
  }
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self {
    self.translate(pos - self.position(),)
  }
  fn translate<Trans,>(&mut self, trans: Trans,) -> &mut Self
    where Trans: Clone, Self::Pos: ops::Add<Trans, Output = Self::Pos>, {
    self.p1.translate(trans.clone(),);
    self.p2.translate(trans.clone(),);
    self.p3.translate(trans,);

    self
  }
}

impl<Num: Sqrt + Clone + Into<f32>,> Orientation<Num,> for Triangle<Num,> {
  fn direction(&self,) -> Vector<Num,> {
    let v1 = self.p2.clone() - self.p1.clone();
    let v2 = self.p3.clone() - self.p1.clone();

    Vector::cross(v1, v2,)
  }
  fn set_direction(&mut self, direction: Vector<Num,>,) -> &mut Self {
    self.rotate(&Rotation::between(self.direction(), direction,),)
  }
  fn rotate(&mut self, rotation: &Rotation,) -> &mut Self {
    self.p1.rotate(rotation,);
    self.p2.rotate(rotation,);
    self.p3.rotate(rotation,);

    self
  }
}
