use crate::{
    cheat::GlobalData,
    classes::{cbaseentity::CBaseEntity, cusercmd::CUserCMD},
};

const IN_ATTACK: i32 = 1 << 0;

pub fn run(global_data: &GlobalData, cmd: &mut CUserCMD) {
    if (cmd.get_buttons() & IN_ATTACK) == IN_ATTACK {
        return;
    }

    // Gets the local entity, so we can read its values.
    let local_entity = CBaseEntity::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(1),
    );

    let mut closest_entity_id = 0;
    let mut closest_entity_distance = f32::MAX;

    // Loop through every player
    for i in 2..32 {
        // Get the entity handle of the player
        let entity = CBaseEntity::new(
            global_data
                .client_entity_list_interface
                .get_client_entity(i),
        );

        // Check if the entity handle is null
        if entity.object_address.is_null() {
            continue;
        }

        // Check if the player is in our team, if it is then we will skip it
        if local_entity.get_team_num() == entity.get_team_num() {
            continue;
        }

        // Calculate the distance between us and the other player
        let distance = local_entity
            .get_origin()
            .calculate_distance(&entity.get_origin());

        // If this player is closer than any player before, then we will set it as the closest one
        if distance < closest_entity_distance {
            closest_entity_id = i;
            closest_entity_distance = distance;
        }
    }

    // Get the closest entity player as a object
    let entity = CBaseEntity::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(closest_entity_id),
    );

    // Check if the entity handle is null
    if entity.object_address.is_null() {
        return;
    }

    // If the closest player is closer than 200 (units ?)
    if closest_entity_distance < 200.0 {
        // Calculate the a view angle where the we look at the closest player
        let view_angle = local_entity
            .get_origin()
            .calculate_angle(&entity.get_origin());

        let differential = view_angle - cmd.get_view_angle();

        // Setting the x value of the view angle to -90 makes backstabbing pretty much impossible, but also scrables your own movement for some reason
        let mut view_angle = cmd.get_view_angle();

        if (differential.y >= 90.0 && differential.y <= 270.0)
            || (differential.y <= -90.0 && differential.y >= -255.0)
        {
            view_angle.x = -90.0;
        }

        // Set the view angle to our new one
        cmd.set_view_angle(view_angle);
    }
}
