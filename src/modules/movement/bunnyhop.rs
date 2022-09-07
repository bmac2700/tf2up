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

    let local_player = CBasePlayer::new(
        global_data
            .client_entity_list_interface
            .get_client_entity(1),
    );

    if (cmd.get_buttons() & IN_JUMP) == IN_JUMP {
        let mut buttons = cmd.get_buttons();

        if !*last_jumped && (local_player.get_flags() & FL_ONGROUND) != FL_ONGROUND {
            buttons &= !IN_JUMP;
        } else if *last_jumped {
            *last_jumped = false;
        }

        cmd.set_buttons(buttons);
    } else if !*last_jumped {
        *last_jumped = true;
    }
}
