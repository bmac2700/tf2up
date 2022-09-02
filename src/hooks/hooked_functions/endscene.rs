use winapi::{
    shared::{
        d3d9::IDirect3DDevice9,
        d3d9types::{D3DCLEAR_TARGET, D3DCOLOR_ARGB, D3DRECT},
    },
    um::winnt::HRESULT,
};

pub extern "thiscall" fn endscene_hook(
    caller_class: *const u8,
    device: *mut IDirect3DDevice9,
) -> HRESULT {
    let global_data = crate::cheat::get_global_data();

    let rectangle = D3DRECT {
        x1: 100,
        x2: 200,
        y1: 100,
        y2: 200,
    };
    unsafe {
        (*device).Clear(
            1,
            &rectangle,
            D3DCLEAR_TARGET,
            D3DCOLOR_ARGB(255, 255, 0, 0),
            0f32,
            0,
        )
    };

    type FunctionSignature = extern "thiscall" fn(*const u8, *mut IDirect3DDevice9) -> HRESULT;
    let func: FunctionSignature =
        unsafe { std::mem::transmute(global_data.endscene_hook as *const u8) };
    return func(caller_class, device);
}
