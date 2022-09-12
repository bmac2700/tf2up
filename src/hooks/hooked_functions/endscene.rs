use winapi::{
    shared::{
        d3d9::{IDirect3DDevice9, LPDIRECT3DVERTEXBUFFER9},
        d3d9types::{D3DCLEAR_TARGET, D3DCOLOR_ARGB, D3DRECT, D3DFVF_XYZRHW, D3DFVF_DIFFUSE, D3DPOOL_DEFAULT, D3DPT_TRIANGLELIST},
    },
    um::winnt::HRESULT,
};

#[derive(Clone, Default)]
pub struct GUIData {
    egui_ctx: egui::CtxRef,
}

use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref GUI_DATA: Mutex<GUIData> = Mutex::new(GUIData::default());
}



pub extern "thiscall" fn endscene_hook(
    caller_class: *const u8,
    device: *mut IDirect3DDevice9,
) -> HRESULT {
    let global_data = crate::cheat::get_global_data();
    let mut gui_data = GUI_DATA.lock().unwrap();

    let raw_input: egui::RawInput = egui::RawInput {
        screen_rect: Some(egui::Rect {
            min: egui::Pos2 { x: 0.0, y: 0.0 },
            max: egui::Pos2 {
                x: 1920.0,
                y: 1080.0,
            },
        }),
        ..Default::default()
    };

    let full_output = gui_data.egui_ctx.run(raw_input, |egui_ctx| {
        //my_app.ui(egui_ctx); // add panels, windows and widgets to `egui_ctx` here
        //egui_ctx.begin_frame(raw_input);

        egui::CentralPanel::default().show(egui_ctx, |ui| {
            ui.heading("My test application");
        });
        //egui_ctx.end_frame();
    });

    let clipped_primitives = gui_data.egui_ctx.tessellate(full_output.1); // creates triangles to paint

    //my_integration.paint(&full_output.textures_delta, clipped_primitives);

    for primitive in clipped_primitives {
        //println!("{:?}", primitive);

        let vertices = primitive.1.vertices;

        unsafe {
            let mut p_vertex_object: LPDIRECT3DVERTEXBUFFER9 = 0 as _;
            
            let result = (*device).CreateVertexBuffer((vertices.len() as usize * 8) as _, 0 as _, D3DFVF_XYZRHW|D3DFVF_DIFFUSE, D3DPOOL_DEFAULT, &mut p_vertex_object, 0 as _);
            println!("CreateVertexBuffer: {}", result);

            let result = (*p_vertex_object).Lock(0, (vertices.len() as usize * 8) as _, p_vertex_object as _, 0);
            println!("Lock: {}", result);

            // Copy to the buffer

            //D3DVertex

            (*p_vertex_object).Unlock();


            // Copy for recovery
            let mut old_vertex_buffer: LPDIRECT3DVERTEXBUFFER9 = 0 as _;
            let mut old_offset = 0;
            let mut old_stride = 0;
            (*device).GetStreamSource(0, &mut old_vertex_buffer, &mut old_offset, &mut old_stride);

            let mut old_fvf = 0;
            (*device).GetFVF(&mut old_fvf);

            // Do the magic

            let result = (*device).SetStreamSource(0, p_vertex_object, 0, 8);
            println!("SetStreamSource: {}", result);

            let result = (*device).SetFVF(D3DFVF_XYZRHW|D3DFVF_DIFFUSE);
            println!("SetFVF: {}", result);

            let result = (*device).DrawPrimitive(D3DPT_TRIANGLELIST, 0, 1);
            println!("DrawPrimitive: {}", result);

            (*p_vertex_object).Release();

            // Recovery
            (*device).SetStreamSource(0, old_vertex_buffer, old_offset, old_stride);
            (*device).SetFVF(old_fvf);
        }
    }

    type FunctionSignature = extern "thiscall" fn(*const u8, *mut IDirect3DDevice9) -> HRESULT;
    let func: FunctionSignature =
        unsafe { std::mem::transmute(global_data.endscene_hook as *const u8) };
    return func(caller_class, device);
}
