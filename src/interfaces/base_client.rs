#[derive(Debug, Clone, Copy, Default)]
pub struct IBaseClient {
    pub interface_address: usize,
}

#[allow(dead_code)]
impl IBaseClient {
    pub fn new(pinterface_address: *const u8) -> Self {
        Self {
            interface_address: pinterface_address as usize,
        }
    }

    pub fn get_all_classes(&self) -> *const u8 {
        let self_address = self.interface_address as *const u8;

        type FunctionSignature = extern "thiscall" fn(*const u8) -> *const u8;
        const FUNC_INDEX: usize = 8;

        let function_address = unsafe {
            *((*(self_address as *const usize) + std::mem::size_of::<usize>() * FUNC_INDEX)
                as *const usize)
        };

        let func: FunctionSignature = unsafe { std::mem::transmute(function_address as *const u8) };

        return func(self_address as _);
    }
}
