use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}};

static mut _HEADERTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__headerText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _HEADERTEXT_FIELD })
}

type SetupRequesterInfoFn = extern "C" fn(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppString);
extern "C" fn SetupRequesterInfo(this: *mut Il2CppObject, typ: *mut Il2CppObject, _headerName: *mut Il2CppString) {
    get_orig_fn!(SetupRequesterInfo, SetupRequesterInfoFn)(this, typ, "hhbbwa".to_string().to_il2cpp_string());
}


pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);

    let SetupRequesterInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupRequesterInfo", 2);
    new_hook!(SetupRequesterInfo_addr, SetupRequesterInfo);

    unsafe {
        _HEADERTEXT_FIELD = get_field_from_name(JukeboxRequestInfo, c"_headerText");
    }
}
