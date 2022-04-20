use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // 3D point

pub struct Ray {
  origin: Vec3,
  direction: Vec3,
}

impl Ray {
  fn origin(&self) -> Vec3 {
    self.origin
  }
  fn direction(&self) -> Vec3 {
    self.direction
  }
  fn at(&self, t: f64) -> Point {
    self.origin() + t * self.direction()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn ray_at() {
    let ray = Ray {
      origin: Vec3(0.0, 0.0, 1.0),
      direction: Vec3(1.0, 1.0, 0.0),
    };

    let point: Point = ray.at(2.0);

    assert_eq!(point.x(), 2.0);
    assert_eq!(point.y(), 2.0);
    assert_eq!(point.z(), 1.0);
  }
}
