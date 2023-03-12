use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // 3D point

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
    fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = *outward_normal * -1.0;
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
