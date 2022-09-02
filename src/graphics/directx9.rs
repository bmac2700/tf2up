use winapi::{
    shared::{
        d3d9::{
            Direct3DCreate9, IDirect3DDevice9, D3DADAPTER_DEFAULT,
            D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3D_SDK_VERSION,
        },
        d3d9types::{D3DDEVTYPE_HAL, D3DPRESENT_PARAMETERS, D3DSWAPEFFECT_DISCARD},
        minwindef::TRUE,
    },
    um::winuser::GetForegroundWindow,
};

pub unsafe fn get_directx_vtable() -> *const u8 {
    let d3d9_device: *mut IDirect3DDevice9 = get_device().unwrap();

    let vtable: *const u8 = (*(d3d9_device as *const usize)) as *const u8;
    (*d3d9_device).Release();

    vtable
}

pub unsafe fn get_device() -> Option<*mut IDirect3DDevice9> {
    let d3d9 = Direct3DCreate9(D3D_SDK_VERSION);

    let window = GetForegroundWindow();

    let mut present_params: D3DPRESENT_PARAMETERS = D3DPRESENT_PARAMETERS {
        BackBufferWidth: 0,
        BackBufferHeight: 0,
        BackBufferFormat: 0,
        BackBufferCount: 0,
        MultiSampleType: 0,
        MultiSampleQuality: 0,
        SwapEffect: D3DSWAPEFFECT_DISCARD,
        hDeviceWindow: window,
        Windowed: TRUE,
        EnableAutoDepthStencil: 0,
        AutoDepthStencilFormat: 0,
        Flags: 0,
        FullScreen_RefreshRateInHz: 0,
        PresentationInterval: 0,
    };

    let d3d9_device: *mut IDirect3DDevice9 = std::ptr::null_mut();
    let result = (*d3d9).CreateDevice(
        D3DADAPTER_DEFAULT,
        D3DDEVTYPE_HAL,
        present_params.hDeviceWindow,
        D3DCREATE_SOFTWARE_VERTEXPROCESSING,
        &mut present_params,
        std::mem::transmute(&d3d9_device),
    );

    (*d3d9).Release(); //Might not be okay, but in this case we are only interested in the virtual table so it doesnt matter

    if result != 0 {
        return None;
    }

    return Some(d3d9_device);
}
