use crate::{cheat::GlobalData, classes::{cusercmd::CUserCMD, cbaseplayer::CBasePlayer}};

pub fn run(global_data: &GlobalData, cmd: &mut CUserCMD) {

    let local_player = CBasePlayer::new(global_data.client_entity_list_interface.get_client_entity(1));

    if (local_player.get_flags(&global_data.netvars) & (1 << 0)) == 0 {
        
        let mut buttons = cmd.get_buttons();
        buttons &= !(1 << 1);

        cmd.set_buttons(buttons);
    }
}