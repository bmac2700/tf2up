use crate::math::vector3::Vec3;

#[derive(Debug)]
pub struct CUserCMD {
    pub object_address: *const u8,
}

#[allow(dead_code)]
impl CUserCMD {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }
}

/*command_number: i32,
tick_count: i32,

view_angles: Vec3,

forward_move: f32,
side_move: f32,
up_move: f32

buttons: i32,

impulse: u8,

weapon_select: i32,
weapon_subtype: i32,

random_seed: i32,

mouse_dx: i16,
mouse_dy: i16,

has_been_predicted: bool,
*/

use crate::class_member;

#[allow(dead_code)]
impl CUserCMD {
    class_member!(command_number, i32, 0x4);
    class_member!(tick_count, Vec3, 0x8);

    class_member!(view_angle, Vec3, 0xC);

    class_member!(forward_move, f32, 0x18);
    class_member!(side_move, f32, 0x1C);
    class_member!(up_move, f32, 0x20);

    class_member!(buttons, i32, 0x24);
}
