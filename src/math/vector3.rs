use super::deg2rad;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn calculate_angle(&self, to: &Vec3) -> Vec3 {
        let delta = Vec3 {
            x: self.x - to.x,
            y: self.y - to.y,
            z: self.z - to.z,
        };

        let rad2deg = 180.0f32 / std::f32::consts::PI;

        let hyp = (delta.x * delta.x + delta.y * delta.y).sqrt();
        let mut angle = Vec3 {
            x: (delta.z / hyp).atan() * rad2deg,
            y: (delta.y / delta.x).atan() * rad2deg,
            z: 0f32,
        };

        if delta.x >= 0.0f32 {
            angle.y += 180f32;
        }

        angle
    }

    pub fn calculate_distance(&self, to: &Vec3) -> f32 {
        let delta = Vec3 {
            x: self.x - to.x,
            y: self.y - to.y,
            z: self.z - to.z,
        };

        let distance = (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt();

        distance
    }
}

pub fn recalculate_viewangle(
    old_angle: Vec3,
    old_forward_move: f32,
    old_side_move: f32,
    cmd_view_angle: Vec3,
) -> (f32, f32) {
    let f1 = if old_angle.y < 0f32 {
        360.0 + old_angle.y
    } else {
        old_angle.y
    };

    let f2 = if cmd_view_angle.y < 0f32 {
        360.0 + cmd_view_angle.y
    } else {
        cmd_view_angle.y
    };

    let delta_view = 360.0
        - if f2 < f1 {
            (f2 - f1).abs()
        } else {
            360.0 - (f1 - f2).abs()
        };

    let new_forward_move = (deg2rad(delta_view)).cos() * old_forward_move
        + (deg2rad(delta_view + 90.0)).cos() * old_side_move;
    let new_side_move = (deg2rad(delta_view)).sin() * old_forward_move
        + (deg2rad(delta_view + 90.0)).sin() * old_side_move;

    (new_forward_move, new_side_move)
}
