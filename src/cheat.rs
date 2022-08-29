use crate::hooks::detour::hook_x86;
use crate::hooks::get_virtual;
use crate::interfaces::{
    base_client::IBaseClient, client_mode::IClientMode, engine_client::IEngineClient, get_interface,
};

#[no_mangle]
pub extern "thiscall" fn create_move_hook(
    caller_class: *const u8,
    input_sample_frametime: f32,
    user_cmd: *const u8,
) {
    println!("Called create_move_hook");
    loop {}
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
        hook_x86(
            get_virtual(client_mode_interface.interface_address, 21),
            6,
            create_move_hook as *const u8,
        );
    }

    let mut view_angle = engine_client_interface.get_view_angle();

    view_angle.y += 60f32;

    engine_client_interface.set_view_angle(view_angle);
}

pub fn end() {}
