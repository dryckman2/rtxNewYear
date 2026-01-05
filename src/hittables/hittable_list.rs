use std::sync::Arc;

use crate::hittables::hittable::{HitRecord, Hittable};
use crate::math::aabb::AABB;
use crate::math::interval::Interval;
use crate::math::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    pub bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::blank(),
        }
    }

    pub fn single(object: Arc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::blank();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                *rec = temp_rec.clone();
                closest_so_far = temp_rec.t;
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
