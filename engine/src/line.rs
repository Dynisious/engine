//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-22

use crate::transform::{Position, Orientation,};
use vector::{Vector, Sqrt, Trigonometry,};

/// A line in space.
#[derive(PartialEq, Eq, Clone, Copy, Hash,)]
pub struct Line<Num,> {
  /// The origin point of the Line.
  pub location: Vector<Num,>,
  /// The direction of the Line.
  pub direction: Vector<Num,>,
}

impl<Num: Clone,> Position for Line<Num,> {
  type Pos = Vector<Num,>;

  #[inline]
  fn position(&self,) -> Self::Pos { self.location.clone() }
  #[inline]
  fn set_position(&mut self, pos: Self::Pos,) -> &mut Self {
    self.location = pos; self
  }
}

impl<Num,> Orientation<Num,> for Line<Num,>
  where Num: Sqrt + Trigonometry + Clone, {
  #[inline]
  fn direction(&self,) -> Vector<Num,> { self.direction.clone() }
  #[inline]
  fn set_direction(&mut self, direction: Vector<Num,>,) -> &mut Self {
    self.direction = direction; self
  }
}
