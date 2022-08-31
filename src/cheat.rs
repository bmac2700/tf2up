use crate::hooks::detour::{hook_x86, unhook_x86};
use crate::hooks::get_virtual;
use crate::interfaces::{
    base_client::IBaseClient, client_mode::IClientMode, engine_client::IEngineClient, get_interface,
};

use crate::hooked_functions::create_move::create_move_hook;

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalData {
    pub base_client_interface: IBaseClient,
    pub engine_client_interface: IEngineClient,
    pub client_mode_interface: IClientMode,

    //hooks
    pub create_move_hook: usize,
}

use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref GLOBAL_DATA: Mutex<GlobalData> = Mutex::new(GlobalData::default());
}

pub fn get_global_data() -> GlobalData {
    let global_data = GLOBAL_DATA.lock().unwrap();

    return *global_data;
}

pub fn start() {
    let mut global_data = GLOBAL_DATA.lock().unwrap();

    global_data.base_client_interface =
        IBaseClient::new(get_interface("client.dll".into(), "VClient017".into()) as _);

    global_data.engine_client_interface =
        IEngineClient::new(get_interface("engine.dll".into(), "VEngineClient014".into()) as _);

    global_data.client_mode_interface =
        IClientMode::new(global_data.base_client_interface.interface_address as *const u8);

    unsafe {
        global_data.create_move_hook = hook_x86(
            get_virtual(
                global_data.client_mode_interface.interface_address as *const u8,
                21,
            ),
            6,
            create_move_hook as *const u8,
        ) as usize;
    }
}

pub fn end() {
    let global_data = GLOBAL_DATA.lock().unwrap();

    unsafe {
        unhook_x86(
            get_virtual(
                global_data.client_mode_interface.interface_address as *const u8,
                21,
            ),
            global_data.create_move_hook as *const u8,
            6,
        );
    }
}
