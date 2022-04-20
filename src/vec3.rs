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

  pub fn dot(&self, _rhs: &Vec3) -> f64 {
    self.x() * _rhs.x() + self.y() * _rhs.y() + self.z() * _rhs.z()
  }

  pub fn cross(&self, _rhs: &Vec3) -> Vec3 {
    return Vec3(
      self.y() * _rhs.z() - self.z() * _rhs.y(),
      self.z() * _rhs.x() - self.x() * _rhs.z(),
      self.x() * _rhs.y() - self.y() * _rhs.x(),
    );
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

impl ops::Mul<f64> for Vec3 {
  type Output = Self;
  fn mul(self, _rhs: f64) -> Self::Output {
    return Self(self.x() * _rhs, self.y() * _rhs, self.z() * _rhs);
  }
}

impl ops::Mul<Vec3> for Vec3 {
  type Output = Self;
  fn mul(self, _rhs: Self) -> Self::Output {
    return Self(
      self.x() * _rhs.x(),
      self.y() * _rhs.y(),
      self.z() * _rhs.z(),
    );
  }
}

impl ops::Mul<Vec3> for f64 {
  type Output = Vec3;
  fn mul(self, _rhs: Vec3) -> Self::Output {
    return _rhs * self;
  }
}

impl ops::DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, other: f64) {
    *self *= 1.0 / other
  }
}

impl ops::Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, _rhs: f64) -> Self::Output {
    return (1.0 / _rhs) * self;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn vec3_add() {
    let v1 = Vec3(1.0, 0.0, 0.0);
    let v2 = Vec3(1.0, 1.0, 0.0);

    let result = v1 + v2;

    assert_eq!(result.x(), 2.0);
  }

  #[test]
  fn vec3_add_assign() {
    let v1 = Vec3(1.0, 0.0, 0.0);
    let mut v3 = Vec3(2.0, 1.0, 0.0);
    v3 += v1;
    assert_eq!(v3.x(), 3.0);
  }

  #[test]
  fn vec3_sub() {
    let v1 = Vec3(1.0, 0.0, 0.0);
    let v2 = Vec3(1.0, 1.0, 0.0);

    let sub_result = v1 - v2;
    assert_eq!(sub_result.x(), 0.0);
  }

  #[test]
  fn vec3_mul() {
    let v1 = Vec3(1.0, 2.0, 1.0);
    let v2 = Vec3(4.0, 3.0, 5.0);

    let result_1 = v1 * v2;
    let result_2 = v1 * 3.0;
    let result_3 = 3.0 * v1;

    assert_eq!(result_1.x(), 4.0);
    assert_eq!(result_2.x(), 3.0);
    assert_eq!(result_3.y(), 6.0);
  }

  #[test]
  fn vec3_mul_assign() {
    let mut v4 = Vec3(1.0, 1.0, 0.0);
    v4 *= 4.0;
    assert_eq!(v4.y(), 4.0);
  }

  #[test]
  fn vec3_div_assign() {
    let mut v5 = Vec3(5.0, 1.0, 0.0);
    v5 /= 5.0;
    assert_eq!(v5.x(), 1.0);
  }

  #[test]
  fn vec3_div() {
    let v1 = Vec3(5.0, 1.0, 0.0);
    let result = v1 / 2.0;
    assert_eq!(result.x(), 2.5);
  }

  #[test]
  fn vec3_length() {
    let v6 = Vec3(1.0, 1.0, 0.0);
    assert_eq!(v6.length(), (1.0 as f64 + 1.0 as f64).sqrt());
  }

  #[test]
  fn vec3_dot() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    let result = v1.dot(&v2);
    assert_eq!(result, 32.0);
  }

  #[test]
  fn vec3_cross() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    let result = v1.cross(&v2);
    assert_eq!(result.x(), -3.0);
    assert_eq!(result.y(), 6.0);
    assert_eq!(result.z(), -3.0);
  }
}
