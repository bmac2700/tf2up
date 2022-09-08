use std::sync::Mutex;

use crate::{
    cheat::GlobalData,
    classes::{cbaseplayer::CBasePlayer, cusercmd::CUserCMD},
};

const IN_JUMP: i32 = 1 << 1;

const FL_ONGROUND: i32 = 1 << 0;

use lazy_static::lazy_static;
lazy_static! {
    static ref LAST_JUMPED: Mutex<bool> = Mutex::new(true);
}

pub fn run(global_data: &GlobalData, cmd: &mut CUserCMD) {
    let mut last_jumped = LAST_JUMPED.lock().unwrap();

    // Gets the local player, so we can read its values.
    let local_player = CBasePlayer::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(1),
    );

    // Checks if the JUMP button is being pressed.
    if (cmd.get_buttons() & IN_JUMP) == IN_JUMP {
        // Saves the current pressed buttons into a mutable variable
        let mut buttons = cmd.get_buttons();

        // If the player has not jumped yet and the player is not on ground, then we will set the buttons to not have the button pressed.
        if !*last_jumped && (local_player.get_flags() & FL_ONGROUND) != FL_ONGROUND {
            buttons &= !IN_JUMP;

            // If the player has jumped we will changed the last_jumped variable from true to false.
        } else if *last_jumped {
            *last_jumped = false;
        }

        // Overwrites the current pressed buttons with the saved buttons.
        cmd.set_buttons(buttons);

        // If the player has not jumped yet, then we will set it so that the player has jumped.
    } else if !*last_jumped {
        *last_jumped = true;
    }
}
