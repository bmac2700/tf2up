#[derive(Debug)]
pub struct CBasePlayer {
    pub object_address: *const u8,
}

#[allow(dead_code)]
impl CBasePlayer {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }
}

use crate::{netvar, netvars::netvar_hash};
use std::collections::HashMap;

#[allow(dead_code)]
impl CBasePlayer {
    netvar!(health, i32, "CBasePlayer->m_iHealth");
    netvar!(life_state, u8, "CBasePlayer->m_lifeState");
    netvar!(flags, i32, "CBasePlayer->m_fFlags");
}
