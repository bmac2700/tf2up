#[derive(Debug)]
pub struct IBaseClient {
    pub interface_address: *const u8,
}

impl IBaseClient {
    pub fn new(interface_address: *const u8) -> Self {
        Self { interface_address }
    }
}
