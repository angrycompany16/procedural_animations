// TODO: turn into crate lol

use std::f32::consts::PI;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

pub fn lerp2(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn sine_in(t: f32) -> f32 {
    1.0 - (t * PI * 0.5).cos()
}

pub fn sine_out(t: f32) -> f32 {
    (t * PI * 0.5).sin()
}

pub fn sine_in_out(t: f32) -> f32 {
    -((PI * t - 1.0) * 0.5).cos()
}

pub fn back_in(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;

    c3 * t * t * t - c1 * t * t
}

pub fn back_out(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;

    1.0 + c3 * (t - 1.0).powf(3.0) + c1 * (t - 1.0).powf(2.0)
}

pub fn back_in_out(t: f32) -> f32 {
    let c1 = 1.70158;
    let c2 = c1 * 1.525;

    if t < 0.5 {
        (2.0 * t).powf(2.0) * ((c2 + 1.0) * 2.0 * t - c2) * 0.5
    } else {
        ((2.0 * t - 2.0).powf(2.0) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) * 0.5
    }
}
