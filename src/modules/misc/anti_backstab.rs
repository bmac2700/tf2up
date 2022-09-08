use crate::{
    cheat::GlobalData,
    classes::{cbaseentity::CBaseEntity, cbaseplayer::CBasePlayer, cusercmd::CUserCMD},
};

pub fn run(global_data: &GlobalData, cmd: &mut CUserCMD) {
    // Gets the local player, so we can read its values.
    let local_player = CBasePlayer::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(1),
    );

    let local_entity = CBaseEntity::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(1),
    );

    for i in 2..32 {
        let entity_ptr = global_data
            .client_entity_list_interface
            .get_client_entity(i);

        if entity_ptr.is_null() {
            continue;
        }

        let entity = CBaseEntity::new(entity_ptr);
        let player = CBasePlayer::new(entity_ptr);

        if local_entity.get_team_num() == entity.get_team_num() {
            continue;
        }

        let distance = local_entity
            .get_origin()
            .calculate_distance(&entity.get_origin());

        if distance < 250.0 {
            let view_angle = entity
                .get_origin()
                .calculate_angle(&local_entity.get_origin());
            cmd.set_view_angle(view_angle);
        }
    }
}
