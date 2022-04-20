use crate::color::Color;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // 3D point

pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
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
  pub fn ray_color(&self) -> Color {
    let unit_direction = self.direction().unit_vector();
    // scale unit to t, where 0 <= t <= 1
    let t = 0.5 * unit_direction.y() + 1.0;
    // linear interpolation aka lerp
    // generate color based on y-coordinate
    return (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0);
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
