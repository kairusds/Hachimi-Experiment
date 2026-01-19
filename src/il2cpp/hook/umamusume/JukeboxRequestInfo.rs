use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}};

type SetupRequesterInfoFn = extern "C" fn(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppString);
extern "C" fn SetupRequesterInfo(this: *mut Il2CppObject, typ: *mut Il2CppObject, _headerName: *mut Il2CppString) {
    get_orig_fn!(SetupRequesterInfo, SetupRequesterInfoFn)(this, typ, "".to_string().to_il2cpp_string());
}

// JukeboxRequestHistoryItem
type SetupTrainerRequestDataFn = extern "C" fn(this: *mut Il2CppObject, musicId: *mut Il2CppObject, name: *mut Il2CppString, requestTime: *mut Il2CppObject);
extern "C" fn SetupTrainerRequestData(this: *mut Il2CppObject, musicId: *mut Il2CppObject, _name: *mut Il2CppString, requestTime: *mut Il2CppObject) {
    get_orig_fn!(SetupTrainerRequestData, SetupTrainerRequestDataFn)(this, musicId, "".to_string().to_il2cpp_string(), requestTime);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);
    get_class_or_return!(umamusume, Gallop, JukeboxRequestHistoryItem);

    let SetupRequesterInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupRequesterInfo", 2);
    new_hook!(SetupRequesterInfo_addr, SetupRequesterInfo);

    let SetupTrainerRequestData_addr = get_method_addr(JukeboxRequestHistoryItem, c"SetupTrainerRequestData", 3);
    new_hook!(SetupTrainerRequestData_addr, SetupTrainerRequestData);
}
