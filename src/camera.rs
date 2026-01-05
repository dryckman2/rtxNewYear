use std::sync::Arc;

use crate::color;
use crate::color::Color;
use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::math::interval::Interval;
use crate::math::ray::Ray;
use crate::math::vec3::{Point3, Vec3};
use crate::multithreaded_renderer;
use crate::multithreaded_renderer::Cord;
use crate::{hittables::hittable::HitRecord, util};
use crossbeam_channel::unbounded;
use rand::prelude::SliceRandom;
use threadpool::ThreadPool;

pub struct CameraSettings {
    pub num_threads: usize,
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
}

#[derive(Clone, Copy)]
pub struct Camera {
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0, 0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
    pixel_samples_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
    defocus_angle: f64,   // Variation angle of rays through each pixel
    num_threads: usize,
    image_width: usize,
    image_height: usize,
    samples_per_pixel: i64,
    max_depth: i64,
}

impl Camera {
    pub fn render(self, world: Arc<HittableList>) {
        let (window_s, window_r) = unbounded();
        let (worker_s, worker_r) = unbounded();

        let pool = ThreadPool::new(self.num_threads + 1);

        pool.execute(move || {
            let mut all_px = vec![];
            for j in 0..self.image_height {
                for i in 0..self.image_width {
                    all_px.push((i, j));
                }
            }

            let mut rng = rand::rng();
            all_px.shuffle(&mut rng);

            for p in all_px {
                worker_s.send(Cord { x: p.0, y: p.1 }).unwrap();
            }
        });

        for _ in 0..self.num_threads {
            let wr = worker_r.clone();
            let ws = window_s.clone();
            let w = world.clone();
            pool.execute(move || {
                while let Ok(cord) = wr.recv() {
                    let mut pixel_color = Color::blank();
                    for _ in 0..self.samples_per_pixel {
                        let r = self.get_ray(cord.x as f64, cord.y as f64);
                        pixel_color += self.ray_color(&r, self.max_depth, &w);
                    }
                    color::write_color(&ws, &cord, &(self.pixel_samples_scale * pixel_color));
                }
            });
        }

        // The `draw` function contains the window and must run on the main thread.
        // This call will block until the user closes the window.
        multithreaded_renderer::draw(self.image_height, self.image_width, window_r);
    }

    pub fn initialize(camera_settings: CameraSettings) -> Camera {
        let mut cam = Camera {
            center: camera_settings.lookfrom,
            pixel00_loc: Point3::blank(),
            pixel_delta_u: Vec3::blank(),
            pixel_delta_v: Vec3::blank(),
            pixel_samples_scale: 1.0 / camera_settings.samples_per_pixel as f64,
            u: Vec3::blank(),
            v: Vec3::blank(),
            w: Vec3::blank(),
            num_threads: camera_settings.num_threads,
            image_width: camera_settings.image_width,
            image_height: (camera_settings.image_width as f64 / camera_settings.aspect_ratio)
                as usize,
            samples_per_pixel: camera_settings.samples_per_pixel,
            max_depth: camera_settings.max_depth,
            defocus_disk_u: Point3::blank(),
            defocus_disk_v: Point3::blank(),
            defocus_angle: camera_settings.defocus_angle,
        };

        // Determine viewport dimensions.
        let theta = util::degrees_to_radians(camera_settings.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * camera_settings.focus_dist;
        let viewport_width = viewport_height * (cam.image_width as f64 / cam.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        cam.w = Vec3::unit_vector(camera_settings.lookfrom - camera_settings.lookat);
        cam.u = Vec3::unit_vector(Vec3::cross(camera_settings.vup, cam.w));
        cam.v = Vec3::cross(cam.w, cam.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * cam.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -&cam.v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        cam.pixel_delta_u = viewport_u / cam.image_width as f64;
        cam.pixel_delta_v = viewport_v / cam.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            cam.center - (camera_settings.focus_dist * cam.w) - viewport_u / 2.0 - viewport_v / 2.0;
        cam.pixel00_loc = viewport_upper_left + 0.5 * (cam.pixel_delta_u + cam.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = camera_settings.focus_dist
            * (util::degrees_to_radians(camera_settings.defocus_angle / 2.0)).tan();
        cam.defocus_disk_u = cam.u * defocus_radius;
        cam.defocus_disk_v = cam.v * defocus_radius;

        cam
    }

    fn ray_color(self, r: &Ray, depth: i64, world: &HittableList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::blank();
        }

        let mut rec = HitRecord::blank();

        if world.hit(r, Interval::new(0.001, util::INFINITY), &mut rec) {
            let mut scattered = Ray::blank();
            let mut attenuation = Color::blank();
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }
            return Color::blank();
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    }

    fn get_ray(self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i + offset.x()) * self.pixel_delta_u)
            + ((j + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = util::random_double();

        return Ray::new_timed(ray_origin, ray_direction, ray_time);
    }

    fn sample_square(self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return Vec3::new(
            util::random_double() - 0.5,
            util::random_double() - 0.5,
            0.0,
        );
    }

    fn defocus_disk_sample(self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        return self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v);
    }
}
