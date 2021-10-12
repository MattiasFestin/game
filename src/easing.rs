use bevy::math::{Quat, Vec2, Vec3};

pub fn scale(y: f32, a: f32) -> f32 {
    y * a
}

pub fn flip(x: f32) -> f32 {
    1.0 - x
}

pub fn clamp(x: f32) -> f32 {
    x.abs()
}

pub fn smooth_start<const N: i32>(x: f32 ) -> f32 {
    x.powi(N)
}

pub fn smooth_stop<const N: i32>(x: f32 ) -> f32 {
    flip(flip(x).powi(N))
}

pub fn mix(y1: f32, y2: f32, blend: f32) -> f32 {
    scale(y1, blend) + scale(y2, flip(blend))
}

pub fn smooth_step<const N: i32>(x: f32) -> f32 {
    mix(smooth_start::<N>(x), smooth_stop::<N>(x), x)
}

pub fn lerp(start: Vec2, end: Vec2, x: f32) -> f32 {
    start.lerp(end, x).y
}

pub fn bias<const N: i32>(x: f32, bias: f32) -> f32 {
    let k = (1.0-bias).powi(N);
    return (x * k) / (x * k - x + 1.0);
}

pub fn asymptotic_averaging(current: f32, target: f32, speed: f32) -> f32 {
    current + (target - current) * speed
}

pub fn asymptotic_averaging_3d(current: Vec3, target: Vec3, speed: f32) -> Vec3 {
    current + (target - current) * speed
}

pub fn asymptotic_averaging_rot(current: Quat, target: Quat, speed: f32) -> Quat {
    current + (target - current) * speed
}