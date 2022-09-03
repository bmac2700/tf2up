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
