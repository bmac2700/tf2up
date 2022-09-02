#[derive(Debug)]
pub struct CBasePlayer {
    pub object_address: *const u8,
}

impl CBasePlayer {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }
}
