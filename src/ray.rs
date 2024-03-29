use crate::color::Color;
use crate::common::INFINITY;
use crate::sphere::{HitRecord, Hittable};
use crate::vec3::Vec3;
use crate::vec3::Vec3 as Point; // 3D point

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin() + t * self.direction()
    }
    pub fn ray_color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(self, 0.0, INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color(1.0, 1.0, 1.0));
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0);
    }

    pub fn hit_sphere(&self, center: Point, radius: f64) -> f64 {
        let oc = self.origin() - center;
        let a = self.direction().length_squared();
        let h = oc.dot(&self.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;
        // no intersection
        if discriminant < 0.0 {
            return -1.0;
        }
        return (-h - discriminant.sqrt()) / a;
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
