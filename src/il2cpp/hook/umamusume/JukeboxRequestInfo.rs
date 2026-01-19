use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}};

static mut _USERNAME_FIELD: *mut FieldInfo = 0 as _;
fn get__userName(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _USERNAME_FIELD })
}

type SetupTrainerInfoFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn SetupTrainerInfo(this: *mut Il2CppObject) {
    // get_orig_fn!(SetupTrainerInfo, SetupTrainerInfoFn)(this);
    let username = get__userName(this);
    Text::set_text(username, "".to_string().to_il2cpp_string());
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);

    let SetupTrainerInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupTrainerInfo", 0);
    new_hook!(SetupTrainerInfo_addr, SetupTrainerInfo);

    unsafe {
        _USERNAME_FIELD = get_field_from_name(JukeboxRequestInfo, c"_userName");
    }
}
