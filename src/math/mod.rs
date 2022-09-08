pub mod vector2;
pub mod vector3;

pub fn deg2rad(deg: f32) -> f32 {
    let deg2rad = std::f32::consts::PI / 180.0f32;

    deg * deg2rad
}
