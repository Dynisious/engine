//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-7

use crate::transform::Position;
use vector::{Number, Vector,};

/// A line in space.
#[derive(PartialEq, Eq, Clone, Copy, Hash,)]
pub struct Line<Num: Number,> {
  /// The origin point of the Line.
  pub location: Vector<Num,>,
  /// The direction of the Line.
  pub direction: Vector<Num,>,
}

impl<Num: Number + Clone,> Position for Line<Num,> {
  type Pos = Vector<Num,>;

  #[inline]
  fn position(&self,) -> Self::Pos { self.location.clone() }
  #[inline]
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self {
    self.location = pos; self
  }
}
