use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}};

static mut _USERNAME_FIELD: *mut FieldInfo = 0 as _;
fn get__userName(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _USERNAME_FIELD })
}

type SetupTrainerInfoFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn SetupTrainerInfo(this: *mut Il2CppObject) {
    // get_orig_fn!(SetupTrainerInfo, SetupTrainerInfoFn)(this);
    let username = get__userName(this);
    Text::set_text(username, "auajaaha".to_string().to_il2cpp_string());
}

type SetupRequesterInfoFn = extern "C" fn(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppObject);
extern "C" fn SetupRequesterInfo(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppObject) {
    // get_orig_fn!(SetupRequesterInfo, SetupRequesterInfoFn)(this, typ, headerName);
    let username = get__userName(this);
    Text::set_text(username, "hhbbwa".to_string().to_il2cpp_string());
}

// type get_RequestUserNameFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppString;
extern "C" fn get_RequestUserName(this: *mut Il2CppObject) -> *mut Il2CppString {
    let username = get__userName(this);
    Text::set_text(username, "nbbaba".to_string().to_il2cpp_string());
    "hihhaaa".to_string().to_il2cpp_string()
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);

    let SetupTrainerInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupTrainerInfo", 0);
    new_hook!(SetupTrainerInfo_addr, SetupTrainerInfo);
    let SetupRequesterInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupRequesterInfo", 2);
    new_hook!(SetupRequesterInfo_addr, SetupRequesterInfo);
    let get_RequestUserName_addr = get_method_addr(JukeboxRequestInfo, c"get_RequestUserName", 0);
    new_hook!(get_RequestUserName_addr, get_RequestUserName);

    unsafe {
        _USERNAME_FIELD = get_field_from_name(JukeboxRequestInfo, c"_headerText");
    }
}
