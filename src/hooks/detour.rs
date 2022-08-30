use winapi::um::memoryapi::{VirtualProtect, VirtualAlloc};
use winapi::um::winnt::{PAGE_EXECUTE_READWRITE, MEM_RESERVE, MEM_COMMIT};

pub unsafe fn hook_x86(
    target_function: *const u8,
    size: usize,
    detour_function: *const u8,
) -> *const u8 {
    let trampoline_address = VirtualAlloc(0 as _, size+5, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);

    //Copy the original data
    for i in 0..size {
        let byte = *((target_function as usize + i) as *const u8);
        *((trampoline_address as usize + i) as *mut u8) = byte;
    }

    let dst = (target_function as usize) as *mut u8;

    let mut old_protect = 0;

    //------------- Modify the original function -------------//
    VirtualProtect(dst as _, size, PAGE_EXECUTE_READWRITE, &mut old_protect);
    core::ptr::write_bytes(dst, 0x90, size); //Overwrite the original function start with NOP
    
    core::ptr::write(dst, 0xE9);

    let relative_detour_address = (detour_function as usize).overflowing_sub(target_function as usize).0 - 5;

    core::ptr::write((dst as usize + 1) as *mut usize, relative_detour_address);

    VirtualProtect(dst as _, size, old_protect, 0 as _);
    //------------- Modify the original function -------------//

    let x = (target_function as usize).overflowing_sub(trampoline_address as usize).0 - 5;
    *((trampoline_address as usize + size) as *mut u8) = 0xE9;

    for byte in x.to_le_bytes().iter().enumerate() {
        *((trampoline_address as usize + size + 1 + byte.0) as *mut u8) = *byte.1;
    }

    return trampoline_address as _;
}
