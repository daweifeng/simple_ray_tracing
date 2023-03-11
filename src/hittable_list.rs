use std::vec::Vec;

use crate::{
    ray::Ray,
    sphere::{HitRecord, Hittable},
    vec3::Vec3,
};

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far: f64 = t_max;

        for object in &self.objects {
            if (object.hit(r, t_min, closest_so_far, rec)) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        return hit_anything;
    }
}
