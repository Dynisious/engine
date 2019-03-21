//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-22

use crate::transform::{Position, Orientation,};
use vector::{Vector, Rotation, Number, Sqrt, Trigonometry,};
use std::ops;

/// A triangle of three points in space.
#[derive(PartialEq, Eq, Clone, Copy, Hash,)]
pub struct Triangle<Num,> {
  /// The first point.
  pub p1: Vector<Num,>,
  /// The second point.
  pub p2: Vector<Num,>,
  /// The third point.
  pub p3: Vector<Num,>,
}

impl<Num: Number + Clone,> Position for Triangle<Num,> {
  type Pos = Vector<Num,>;

  fn position(&self,) -> Self::Pos {
    (self.p1.clone() + self.p2.clone() + self.p3.clone()) / Number::from_isize(3,)
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

impl<Num,> Orientation<Num,> for Triangle<Num,>
  where Num: Sqrt + Trigonometry + Clone, {
  fn direction(&self,) -> Vector<Num,> {
    let v1 = self.p2.clone() - self.p1.clone();
    let v2 = self.p3.clone() - self.p1.clone();

    Vector::cross(v1, v2,)
  }
  fn set_direction(&mut self, direction: Vector<Num,>,) -> &mut Self {
    self.rotate(&Rotation::between(self.direction(), direction,),)
  }
  fn rotate(&mut self, rotation: &Rotation<Num,>,) -> &mut Self {
    self.p1.rotate(rotation,);
    self.p2.rotate(rotation,);
    self.p3.rotate(rotation,);

    self
  }
}
