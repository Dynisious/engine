//! Author --- daniel.bechaz@gmail.com  
//! Last Moddified --- 2019-03-21

use crate::*;
use std::ops;

/// A rotation in 3D space.
#[derive(Clone, Copy, Debug,)]
pub struct Rotation<Num: Number> {
  /// The axis around which the rotation occours.
  pub axis: Unit<Num,>,
  /// The angle of the rotation.
  pub angle: f32,
}

impl<Num: Number,> Rotation<Num,> {
  /// Creates a new [Rotation] value.
  #[inline]
  pub const fn new(axis: Unit<Num,>, angle: f32,) -> Self { Self { axis, angle, } }
  /// Finds the rotation to go from `from` to `to`.
  /// 
  /// Differences in the length of the [Vector]s have no affect on the [Rotation] returned.
  /// 
  /// # Params
  /// 
  /// from --- The [Vector] to find the [Rotation] from.  
  /// to --- The [Vector] to rotate `from` into.  
  pub fn find_rotation(from: Vector<Num,>, to: Vector<Num,>,) -> Self
    where Num: Sqrt + Clone + Into<f32>, {
    let dot = from.clone() * to.clone();
    //Using the square of the dot means we can use the square of the magnituids while
    //calculating the angle.
    let dot2 = dot.clone() * dot;
    let from_mag2 = from.clone() * from.clone();
    let to_mag2 = to.clone() * to.clone();
    let angle = (dot2.into() / (from_mag2 * to_mag2).into()).acos();
    let axis = Vector::cross(from, to,).into();

    Self { axis, angle, }
  }
}

#[cfg(test,)]
mod tests {
  use super::*;

  #[test]
  fn test_rotation() {
    let x = (1.0, 0.0, 0.0,).into();
    let y = (0.0, 1.0, 0.0,).into();
    let z = (0.0, 0.0, 1.0,).into();

    const PI2: f32 = std::f32::consts::FRAC_PI_2;

    let rot = Rotation::find_rotation(x, y,);
    assert_eq!(rot.axis, z, "Rotation axis is wrong",);
    assert_eq!(rot.angle, PI2, "Rotation angle is wrong",);

    let rot = Rotation::find_rotation(y, z,);
    assert_eq!(rot.axis, x, "Rotation axis is wrong",);
    assert_eq!(rot.angle, PI2, "Rotation angle is wrong",);

    let rot = Rotation::find_rotation(z, x,);
    assert_eq!(rot.axis, y, "Rotation axis is wrong",);
    assert_eq!(rot.angle, PI2, "Rotation angle is wrong",);
  }
}
