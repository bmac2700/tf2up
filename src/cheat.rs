use winapi::um::winuser::VK_OEM_MINUS;

use crate::interfaces::{get_interface, engine_client};

use crate::math::vector3::Vec3;
use crate::interfaces::engine_client::IEngineClient;

pub fn start() {
    let engine_client_interface: IEngineClient = IEngineClient::new(get_interface("engine.dll".into(), "VEngineClient014".into()) as _);

    let mut view_angle = engine_client_interface.get_view_angle();

    view_angle.y += 60f32;
    view_angle.y.clamp(-180.0, 180.0);

    engine_client_interface.set_view_angle(view_angle);
}

pub fn end() {}
