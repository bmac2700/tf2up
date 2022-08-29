#![feature(abi_thiscall)]

use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, DWORD, HMODULE, LPVOID, TRUE},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread},
        processthreadsapi::CreateThread,
        winnt::DLL_PROCESS_ATTACH,
        winuser::{GetAsyncKeyState, VK_END},
    },
};

mod cheat;
mod interfaces;
mod math;
mod hooks;

unsafe extern "system" fn dllmain_wrapped(module: *mut c_void) -> u32 {
    let res = std::panic::catch_unwind(|| {
        winapi::um::consoleapi::AllocConsole();
        winapi::um::wincon::SetConsoleTitleA(b"tf2up\0".as_ptr() as _);

        cheat::start();

        while GetAsyncKeyState(VK_END) == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        cheat::end();

        winapi::um::wincon::FreeConsole();
        FreeLibraryAndExitThread(module as _, 0);
    });

    match res {
        Err(e) => println!("Error: {:?}", e),
        _ => {}
    };

    0
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "system" fn DllMain(module: HMODULE, reason: DWORD, _: LPVOID) -> BOOL {
    /* Disable thread calls */
    DisableThreadLibraryCalls(module);

    /* If we're attaching, create a new thread */
    if reason == DLL_PROCESS_ATTACH {
        /* NOTE: leaking handle here, need to find a way to resolve */
        CloseHandle(CreateThread(
            std::ptr::null_mut(),
            0,
            Some(dllmain_wrapped),
            module as *mut _,
            0,
            std::ptr::null_mut(),
        ));
    }

    /* return true always */
    TRUE
}

#[cfg(test)]
mod tests {}
