use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

pub unsafe fn hook_x86(
    target_function: *const u8,
    size: usize,
    detour_function: *const u8,
) -> *const u8 {
    let mut trampoline: Vec<u8> = Vec::new();


    //Copy the original data
    for i in 0..size {
        trampoline.push(*((target_function as usize + i) as *const u8));
    }

    let dst = (target_function as usize) as *mut u8;

    let mut old_protect = 0;

    VirtualProtect(dst as _, size, PAGE_EXECUTE_READWRITE, &mut old_protect);
    core::ptr::write_bytes(dst, 0x90, size); //Overwrite the original function start with NOP
    
    core::ptr::write(dst, 0xE9);

    let relative_detour_address = (detour_function as usize).overflowing_sub(target_function as usize).0 - 5;

    core::ptr::write((dst as usize + 1) as *mut usize, relative_detour_address);

    VirtualProtect(dst as _, size, old_protect, 0 as _);

    let x = (target_function as usize).overflowing_sub(trampoline.as_ptr() as usize).0 - 5;
    trampoline.push(0xE9);
    for byte in x.to_le_bytes() {
        trampoline.push(byte);
    }

    //HOOK BROKEN

    println!("trampoline instructions: {:x?}", trampoline.as_ptr());

    trampoline.leak();

    return 0 as _;
}
