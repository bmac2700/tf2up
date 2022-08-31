use crate::math::vector3::Vec3;

#[derive(Debug)]
pub struct CUserCMD {
    object_address: *const u8,
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

impl CUserCMD {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }

    pub fn get_command_number(&self) -> i32 {
        let command_number: *const i32 = (self.object_address as usize + 0x4) as _;
        return unsafe { *command_number };
    }

    pub fn get_tickcount(&self) -> i32 {
        let tick_count: *const i32 = (self.object_address as usize + 0x8) as _;
        return unsafe { *tick_count };
    }

    pub fn get_view_angle(&self) -> Vec3 {
        let view_angle: *const Vec3 = (self.object_address as usize + 0xC) as _;
        return unsafe { *view_angle };
    }

    pub fn get_forward_move(&self) -> f32 {
        let forward_move: *const f32 = (self.object_address as usize + 0x18) as _;
        return unsafe { *forward_move };
    }

    pub fn get_side_move(&self) -> f32 {
        let side_move: *const f32 = (self.object_address as usize + 0x1C) as _;
        return unsafe { *side_move };
    }

    pub fn get_up_move(&self) -> f32 {
        let up_move: *const f32 = (self.object_address as usize + 0x20) as _;
        return unsafe { *up_move };
    }

    pub fn get_buttons(&self) -> i32 {
        let buttons: *const i32 = (self.object_address as usize + 0x24) as _;
        return unsafe { *buttons };
    }
}

impl CUserCMD {
    pub fn set_command_number(&self, new_command_number: i32) {
        let command_number: *mut i32 = (self.object_address as usize + 0x4) as _;
        unsafe { *command_number = new_command_number };
    }

    pub fn set_tick_count(&self, new_tick_count: i32) {
        let tick_count: *mut i32 = (self.object_address as usize + 0x8) as _;
        unsafe { *tick_count = new_tick_count };
    }

    pub fn set_view_angle(&self, new_view_angle: Vec3) {
        let view_angle: *mut Vec3 = (self.object_address as usize + 0xC) as _;
        unsafe { *view_angle = new_view_angle };
    }
}
