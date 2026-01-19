use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::get_method_addr, types::*}};

type SetupTrainerInfoFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn SetupTrainerInfo(this: *mut Il2CppObject) {
    get_orig_fn!(SetupTrainerInfo, SetupTrainerInfoFn)(this);
    Text::set_text(this, "".to_string().to_il2cpp_string());
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);

    let SetupTrainerInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupTrainerInfo", 0);
    new_hook!(SetupTrainerInfo_addr, SetupTrainerInfo);
}