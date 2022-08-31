#[derive(Debug, Clone, Copy, Default)]
pub struct IBaseClient {
    pub interface_address: usize,
}

impl IBaseClient {
    pub fn new(pinterface_address: *const u8) -> Self {
        Self {
            interface_address: pinterface_address as usize,
        }
    }
}
