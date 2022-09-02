#[derive(Debug)]
pub struct CBaseEntity {
    pub object_address: *const u8,
}

impl CBaseEntity {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }
}
