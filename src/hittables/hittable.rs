use std::sync::Arc;

use crate::material::material::{Lambertian, Material};
use crate::math::aabb::AABB;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn blank() -> HitRecord {
        HitRecord {
            p: Point3::blank(),
            normal: Vec3::blank(),
            mat: Arc::new(Lambertian::new(Vec3::blank())),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = Vec3::dot(&r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AABB;
}
