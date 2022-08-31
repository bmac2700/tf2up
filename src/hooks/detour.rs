use winapi::um::memoryapi::{VirtualAlloc, VirtualFree, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

pub unsafe fn hook_x86(
    target_function: *const u8,
    size: usize,
    detour_function: *const u8,
) -> *const u8 {
    let gateway_addres = VirtualAlloc(
        0 as _,
        size + 5,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );

    //Copy the original data
    for i in 0..size {
        let byte = *((target_function as usize + i) as *const u8);
        *((gateway_addres as usize + i) as *mut u8) = byte;
    }

    let dst = (target_function as usize) as *mut u8;

    let mut old_protect = 0;

    //------------- Modify the original function -------------//
    VirtualProtect(dst as _, size, PAGE_EXECUTE_READWRITE, &mut old_protect);
    core::ptr::write_bytes(dst, 0x90, size); //Overwrite the original function start with NOP

    core::ptr::write(dst, 0xE9);

    let relative_detour_address = (detour_function as usize)
        .overflowing_sub(target_function as usize)
        .0
        - 5;

    core::ptr::write((dst as usize + 1) as *mut usize, relative_detour_address);

    VirtualProtect(dst as _, size, old_protect, 0 as _);
    //------------- Modify the original function -------------//

    let x = (target_function as usize)
        .overflowing_sub(gateway_addres as usize)
        .0
        - 5;
    *((gateway_addres as usize + size) as *mut u8) = 0xE9;

    for byte in x.to_le_bytes().iter().enumerate() {
        *((gateway_addres as usize + size + 1 + byte.0) as *mut u8) = *byte.1;
    }

    return gateway_addres as _;
}

pub unsafe fn unhook_x86(target_function: *const u8, gateway_address: *const u8, size: usize) {
    let mut old_protect = 0;

    //------------- Restore the original function -------------//
    VirtualProtect(
        target_function as _,
        size,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );

    for i in 0..size {
        *((target_function as usize + i) as *mut u8) =
            *((gateway_address as usize + i) as *const u8);
    }

    VirtualProtect(target_function as _, size, old_protect, 0 as _);
    //------------- Restore the original function -------------//

    VirtualFree(gateway_address as _, size + 5, MEM_RELEASE);
}
