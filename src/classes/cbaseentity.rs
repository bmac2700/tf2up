#[derive(Debug)]
pub struct CBaseEntity {
    pub object_address: *const u8,
}

#[allow(dead_code)]
impl CBaseEntity {
    pub fn new(object_address: *const u8) -> Self {
        Self { object_address }
    }
}

use crate::{math::vector3::Vec3, netvar, netvars::netvar_hash};
use std::collections::HashMap;

#[allow(dead_code)]
impl CBaseEntity {
    netvar!(simulation_time, f32, "CBaseEntity->m_flSimulationTime");
    netvar!(origin, Vec3, "CBaseEntity->m_vecOrigin");
    netvar!(team_num, i32, "CBaseEntity->m_iTeamNum");
}
