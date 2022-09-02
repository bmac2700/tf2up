use crate::hooks::detour::{hook_x86, unhook_x86};
use crate::hooks::get_virtual;
use crate::interfaces::{
    base_client::IBaseClient, client_mode::IClientMode, engine_client::IEngineClient, get_interface,
};

use crate::hooks::hooked_functions::{create_move::create_move_hook, endscene::endscene_hook};
use crate::{graphics, netvars};

#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalData {
    pub base_client_interface: IBaseClient,
    pub engine_client_interface: IEngineClient,
    pub client_mode_interface: IClientMode,

    //hooks
    pub create_move_hook: usize,

    pub endscene_hook: usize,
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

    println!("{:x?}", global_data.base_client_interface.get_all_classes());
    netvars::setup_netvars(global_data.base_client_interface.get_all_classes());

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

    let directx_vtable = unsafe { graphics::directx9::get_directx_vtable() };

    println!("directx vtable: {:x?}", directx_vtable);

    let endscene_address =
        unsafe { *((directx_vtable as usize + 0xA8) as *const usize) };

    println!("endscene: {:x?}", endscene_address);

    unsafe {
        global_data.endscene_hook =
            hook_x86(endscene_address as *const u8, 7, endscene_hook as *const u8) as usize;
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

    unsafe {
        let endscene_address =
            *((graphics::directx9::get_directx_vtable() as usize + 0xA8) as *const usize);
        unhook_x86(
            endscene_address as *const u8,
            global_data.endscene_hook as *const u8,
            7,
        );
    }
}
