//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-7

use crate::transform::Position;
use vector::{Number, Vector,};
use std::ops::Add;

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

  #[inline]
  fn position(&self,) -> Self::Pos {
    (self.p1.clone() + self.p2.clone() + self.p3.clone()) / 3.into()
  }
  #[inline]
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self {
    self.translate(pos - self.position(),)
  }
  #[inline]
  fn translate<Trans,>(&mut self, trans: Trans,) -> &mut Self
    where Trans: Clone, Self::Pos: Add<Trans, Output = Self::Pos>, {
    self.p1.translate(trans.clone(),);
    self.p2.translate(trans.clone(),);
    self.p3.translate(trans,);

    self
  }
}
