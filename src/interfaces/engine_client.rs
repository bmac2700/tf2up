use crate::math::vector3::Vec3;

pub struct IEngineClient {
    interface_address: *const u8,
}

impl IEngineClient {
    pub fn new(interface_address: *const u8) -> Self {
        Self {
            interface_address,
        }
    }

    pub fn is_connected(&self) -> u8 {
        type FunctionSignature = extern "thiscall" fn(*const u8) -> u8;
        const FUNC_INDEX: usize = 27;

        let function_address = unsafe {
            *((*(self.interface_address as *const usize)
                + std::mem::size_of::<usize>() * FUNC_INDEX) as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        return func(self.interface_address as _);
    }

    pub fn get_screen_size(&self) -> (i32, i32) {
        let mut width = 0i32;
        let mut height = 0i32;

        type FunctionSignature = extern "thiscall" fn(*const u8, &mut i32, &mut i32);
        const FUNC_INDEX: usize = 5;

        let function_address = unsafe {
            *((*(self.interface_address as *const usize)
                + std::mem::size_of::<usize>() * FUNC_INDEX) as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        func(self.interface_address, &mut width, &mut height);
        
        return (width, height);
    }

    pub fn get_view_angle(&self) -> Vec3 {
        let mut view_angle = Vec3 {
            x: 0f32,
            y: 0f32,
            z: 0f32
        };

        type FunctionSignature = extern "thiscall" fn(*const u8, &mut Vec3);
        const FUNC_INDEX: usize = 19;

        let function_address = unsafe {
            *((*(self.interface_address as *const usize)
                + std::mem::size_of::<usize>() * FUNC_INDEX) as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        func(self.interface_address as _, &mut view_angle);

        return view_angle;
    }

    pub fn set_view_angle(&self, view_angle: Vec3) {
        type FunctionSignature = extern "thiscall" fn(*const u8, &Vec3) -> u8;
        const FUNC_INDEX: usize = 20;

        let function_address = unsafe {
            *((*(self.interface_address as *const usize)
                + std::mem::size_of::<usize>() * FUNC_INDEX) as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        func(self.interface_address as _, &view_angle);
    }
}