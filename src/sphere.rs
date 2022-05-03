use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // 3D point

pub struct Sphere {
  center: Point,
  radius: f64,
}

pub struct HitRecord {
  point: Point,
  normal: Vec3,
  t: f64,
  front_face: bool,
}

impl HitRecord {
  fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: &Vec3) {
    self.front_face = ray.direction().dot(outward_normal) < 0.0;
    if self.front_face {
      self.normal = *outward_normal
    } else {
      self.normal = *outward_normal * -1.0;
    }
  }
}

impl Sphere {
  pub fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc = ray.origin() - self.center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
      return false;
    } else {
      let sqrtd = discriminant.sqrt();

      // Find the nearest root that lines in the acceptable range
      let mut root = (-half_b - sqrtd) / a;
      if root < t_min || t_max < root {
        root = (-half_b + sqrtd) / a;
        if root < t_min || t_max < root {
          return false;
        }
      }
      rec.t = root;
      rec.point = ray.at(rec.t);
      let outward_normal = (rec.point - self.center) / self.radius;
      rec.set_face_normal(ray, &outward_normal);
      rec.normal = (rec.point - self.center) / self.radius;

      return true;
    }
  }
}
