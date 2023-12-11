use std::f32::consts::PI;

pub fn shake_animation(t: f32, a: f32) -> i32 {
    // Damped sine wave
    let w = 0.8; // period
    let c = 0.35; // damp factor
    (a * (-c * t).exp() * (w * t).sin()).round() as i32
}