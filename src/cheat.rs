use crate::interfaces::{get_interface, base_client::IBaseClient, client_mode::IClientMode, engine_client::IEngineClient };
use crate::hooks::get_virtual;

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

    let mut view_angle = engine_client_interface.get_view_angle();

    view_angle.y += 60f32;

    engine_client_interface.set_view_angle(view_angle);
}

pub fn end() {}
