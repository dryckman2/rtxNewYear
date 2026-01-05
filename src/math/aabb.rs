use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::Point3;

#[derive(Clone, Copy)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub fn blank() -> AABB {
        AABB {
            x: Interval::blank(),
            y: Interval::blank(),
            z: Interval::blank(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        AABB { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> AABB {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };
        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };
        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };
        AABB { x, y, z }
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> AABB {
        AABB {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        return match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        };
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        let mut ray_t = ray_t.clone();
        for axis in 0..3 {
            let axis = axis as usize;
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / r.direction[axis as usize];

            let t0 = (ax.min - r.origin[axis]) * adinv;
            let t1 = (ax.max - r.origin[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        return true;
    }

    pub fn longest_axis(&self) -> i64 {
        // Returns the index of the longest axis of the bounding box.
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }
}
