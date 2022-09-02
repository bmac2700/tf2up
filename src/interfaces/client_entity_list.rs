#[derive(Debug, Clone, Copy, Default)]
pub struct IClientEntityList {
    pub interface_address: usize,
}

#[allow(dead_code)]
impl IClientEntityList {
    pub fn new(pinterface_address: *const u8) -> Self {
        Self {
            interface_address: pinterface_address as usize,
        }
    }

    pub fn get_client_entity(&self, entnum: i32) -> *const u8 {
        let self_address = self.interface_address as *const u8;

        type FunctionSignature = extern "thiscall" fn(*const u8, i32) -> *const u8;
        const FUNC_INDEX: usize = 3;

        let function_address = unsafe {
            *((*(self_address as *const usize) + std::mem::size_of::<usize>() * FUNC_INDEX)
                as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        return func(self_address as _, entnum);
    }

    pub fn get_highest_entity_index(&self) -> i32 {
        let self_address = self.interface_address as *const u8;

        type FunctionSignature = extern "thiscall" fn(*const u8) -> i32;
        const FUNC_INDEX: usize = 6;

        let function_address = unsafe {
            *((*(self_address as *const usize) + std::mem::size_of::<usize>() * FUNC_INDEX)
                as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        return func(self_address as _);
    }
}
