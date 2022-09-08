use crate::{classes::cusercmd::CUserCMD, math::vector3::recalculate_viewangle};

pub extern "thiscall" fn create_move_hook(
    caller_class: *const u8,
    input_sample_frametime: f32,
    user_cmd: *const u8,
) -> bool {
    let global_data = crate::cheat::get_global_data();

    type FunctionSignature = extern "thiscall" fn(*const u8, f32, *const u8) -> bool;
    let func: FunctionSignature =
        unsafe { std::mem::transmute(global_data.create_move_hook.clone() as *const u8) };
    let original_return_value = func(caller_class, input_sample_frametime, user_cmd);

    let mut cmd = CUserCMD::new(user_cmd);

    if user_cmd as usize == 0 || cmd.get_command_number() == 0 {
        return original_return_value;
    }

    let old_viewangle = cmd.get_view_angle();
    let old_forward_move = cmd.get_forward_move();
    let old_side_move = cmd.get_side_move();

    crate::modules::movement::bunnyhop::run(&global_data, &mut cmd);
    crate::modules::misc::anti_backstab::run(&global_data, &mut cmd);

    let (forward_move, side_move) = recalculate_viewangle(
        old_viewangle,
        old_forward_move,
        old_side_move,
        cmd.get_view_angle(),
    );

    let forward_move = forward_move.clamp(-450.0, 450.0);
    let side_move = side_move.clamp(-450.0, 450.0);

    cmd.set_forward_move(forward_move);
    cmd.set_side_move(side_move);

    return false;
}
