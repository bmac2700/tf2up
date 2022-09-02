use std::ffi::CString;

use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

pub fn get_interface(module_name: String, interface_name: String) -> *const u8 {
    let module_name = CString::new(module_name).unwrap();
    let interface_name = CString::new(interface_name).unwrap();

    let module_handle = unsafe { GetModuleHandleA(module_name.as_ptr() as _) };
    let create_interface_address =
        unsafe { GetProcAddress(module_handle, "CreateInterface\0".as_ptr() as _) };

    type CreateInterfaceFunction = extern "C" fn(*const u8, *const i32) -> *const u8;

    let func: CreateInterfaceFunction = unsafe { std::mem::transmute(create_interface_address) };

    let return_value: i32 = 1;

    let x = func(interface_name.as_ptr() as _, &return_value);

    return x;
}

pub mod base_client;
pub mod client_entity_list;
pub mod client_mode;
pub mod engine_client;
