use std::cmp::Ordering;
use std::sync::Arc;

use crate::hittables::hittable::HitRecord;
use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::math::aabb::AABB;
use crate::math::interval::Interval;
use crate::math::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn from_hittable_list(list: &mut HittableList) -> BvhNode {
        let l = list.objects.len();
        BvhNode::new(&mut list.objects, 0, l)
    }

    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let mut bbox: AABB = AABB::EMPTY.clone();

        // Build the bounding box of the span of source objects.
        for object_index in start..end {
            bbox = AABB::from_boxes(&bbox, objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_span = end - start;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(|a, b| comparator(Arc::clone(a), Arc::clone(b)));

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::new(objects, start, mid));
            right = Arc::new(BvhNode::new(objects, mid, end));
        }

        BvhNode { left, right, bbox }
    }

    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);

        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, 0);
    }

    fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, 1);
    }

    fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> Ordering {
        return Self::box_compare(a, b, 2);
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, &ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let x = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, x), rec);

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
