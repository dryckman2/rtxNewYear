use crate::math::interval::Interval;
use crate::math::vec3::Vec3;
use crate::multithreaded_renderer::{Cord, Pixel};
use crossbeam_channel::Sender;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    return 0.0;
}

pub fn write_color(rx: &Sender<Pixel>, cord: &Cord, pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i64;
    let gbyte = (256.0 * intensity.clamp(g)) as i64;
    let bbyte = (256.0 * intensity.clamp(b)) as i64;

    let p = Pixel {
        cord: *cord,
        color: (rbyte as u8, gbyte as u8, bbyte as u8),
    };

    rx.send(p).unwrap();
}
