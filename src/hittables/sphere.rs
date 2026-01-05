use crate::hittables::hittable::{HitRecord, Hittable};
use crate::material::material::Material;
use crate::math::aabb::AABB;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: Ray::new(static_center, Vec3::blank()),
            radius: radius,
            mat: mat,
            bbox: AABB::from_points(static_center - rvec, static_center + rvec),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: Arc<dyn Material>,
    ) -> Sphere {
        let center_ray = Ray::new(center1, center2 - center1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_points(center_ray.at(0.0) - rvec, center_ray.at(0.0) + rvec);
        let box2 = AABB::from_points(center_ray.at(1.0) - rvec, center_ray.at(1.0) + rvec);
        Sphere {
            center: center_ray,
            radius,
            mat,
            bbox: AABB::from_boxes(&box1, &box2),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}
