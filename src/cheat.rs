use crate::hooks::detour::hook_x86;
use crate::hooks::get_virtual;
use crate::interfaces::{
    base_client::IBaseClient, client_mode::IClientMode, engine_client::IEngineClient, get_interface,
};
use crate::math::vector3::Vec3;

use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref hook_gateway: Mutex<usize> = Mutex::new(0usize);
}

#[derive(Debug)]
#[repr(C)]
struct CUserCMD {
    object_address: *const u8
    /*command_number: i32,
    tick_count: i32,
    
    view_angles: Vec3,

    forward_move: f32,
    side_move: f32,
    up_move: f32*/
}

impl CUserCMD {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }

    pub fn get_command_number(&self) -> i32 {
        let command_number: *const i32 = (self.object_address as usize + 0) as _;
        return unsafe { *command_number };
    }

    pub fn get_tickcount(&self) -> i32 {
        let tick_count: *const i32 = (self.object_address as usize + 4) as _;
        return unsafe { *tick_count };
    }

    pub fn get_view_angle(&self) -> Vec3 {
        let view_angle: *const Vec3 = (self.object_address as usize + 8) as _;
        return unsafe { *view_angle };
    }
}

#[no_mangle]
pub extern "thiscall" fn create_move_hook(
    caller_class: *const u8,
    input_sample_frametime: f32,
    user_cmd: *const u8,
) -> bool {
    println!("Called create_move_hook by {:x?}", caller_class);

    type FunctionSignature = extern "thiscall" fn(*const u8, f32, *const u8) -> bool;
    let func: FunctionSignature = unsafe { std::mem::transmute(hook_gateway.lock().unwrap().clone() as *const u8) };
    let original_return_value = func(caller_class, input_sample_frametime, user_cmd);

    let cmd = CUserCMD::new(user_cmd);

    if user_cmd as usize == 0 || cmd.get_command_number() == 0 || cmd.get_tickcount() == 0 {
        return original_return_value;
    }

    println!("CUserCMD Address: {:x?}", user_cmd);

    println!("command_number: {}\ntick_count: {}\nview_angle: {:?}\n\n\n", cmd.get_command_number(), cmd.get_tickcount(), cmd.get_view_angle());

    return false;
}

pub fn start() {
    let base_client_interface: IBaseClient =
        IBaseClient::new(get_interface("client.dll".into(), "VClient017".into()) as _);
    let engine_client_interface: IEngineClient =
        IEngineClient::new(get_interface("engine.dll".into(), "VEngineClient014".into()) as _);

    let client_mode_interface: IClientMode =
        IClientMode::new(base_client_interface.interface_address);

    println!("base_client_interface: {:x?}", base_client_interface);
    println!("engine_client_interface: {:x?}", engine_client_interface);
    println!("client_mode_interface: {:x?}", client_mode_interface);

    println!("create_move function: {:x?}", unsafe {
        get_virtual(client_mode_interface.interface_address, 21)
    });

    println!("{:x?}", create_move_hook as *const u8);

    unsafe {
        *hook_gateway.lock().unwrap() = hook_x86(
            get_virtual(client_mode_interface.interface_address, 21),
            6,
            create_move_hook as *const u8,
        ) as usize;
    }

    let mut view_angle = engine_client_interface.get_view_angle();

    view_angle.y += 60f32;

    engine_client_interface.set_view_angle(view_angle);
}

pub fn end() {}
