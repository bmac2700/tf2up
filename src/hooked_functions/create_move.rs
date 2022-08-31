use crate::classes::cusercmd::CUserCMD;

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

    let cmd = CUserCMD::new(user_cmd);

    if user_cmd as usize == 0 || cmd.get_command_number() == 0 {
        return original_return_value;
    }

    cmd.set_forward_move(450f32);

    return false;
}
