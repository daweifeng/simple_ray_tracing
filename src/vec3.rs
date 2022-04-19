use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
  pub fn x(&self) -> f64 {
    self.0
  }

  pub fn y(&self) -> f64 {
    self.1
  }

  pub fn z(&self) -> f64 {
    self.2
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
    self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
  }
}

impl ops::Sub<Vec3> for Vec3 {
  type Output = Self;
  fn sub(self, _rhs: Vec3) -> Self::Output {
    return Self(
      self.x() - _rhs.x(),
      self.y() - _rhs.y(),
      self.z() - _rhs.z(),
    );
  }
}

impl ops::Add<Vec3> for Vec3 {
  type Output = Self;
  fn add(self, _rhs: Vec3) -> Self::Output {
    return Self(
      self.x() + _rhs.x(),
      self.y() + _rhs.y(),
      self.z() + _rhs.z(),
    );
  }
}

impl ops::AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, other: Self) {
    *self = Self(
      self.x() + other.x(),
      self.y() + other.y(),
      self.z() + other.z(),
    )
  }
}

impl ops::MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, other: f64) {
    *self = Self(self.x() * other, self.y() * other, self.z() * other)
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) {
    *self *= 1.0 / other
  }
}

#[test]
fn vec3_tests() {
  let v1 = Vec3(1.0, 0.0, 0.0);
  let v2 = Vec3(1.0, 1.0, 0.0);

  let sub_result = v1 - v2;
  let add_result = v1 + v2;
  assert_eq!(sub_result.x(), 0.0);
  assert_eq!(add_result.x(), 2.0);
  let mut v3 = Vec3(2.0, 1.0, 0.0);
  v3 += v1;
  assert_eq!(v3.x(), 3.0);

  let mut v4 = Vec3(1.0, 1.0, 0.0);
  v4 *= 4.0;
  assert_eq!(v4.y(), 4.0);

  let mut v5 = Vec3(5.0, 1.0, 0.0);
  v5 /= 5.0;
  assert_eq!(v5.x(), 1.0);

  let mut v6 = Vec3(1.0, 1.0, 0.0);
  assert_eq!(v6.length(), (1.0 as f64 + 1.0 as f64).sqrt());
}
