use crate::{core::Hachimi, il2cpp::{ext::StringExt, symbols::get_method_addr, types::*}};

type SetupRequesterInfoFn = extern "C" fn(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppString);
extern "C" fn SetupRequesterInfo(this: *mut Il2CppObject, typ: *mut Il2CppObject, headerName: *mut Il2CppString) {
    let header_name = if Hachimi::instance().config.load().hide_usernames {
        "".to_string().to_il2cpp_string();
    } else {
        headerName
    };
    get_orig_fn!(SetupRequesterInfo, SetupRequesterInfoFn)(this, typ, header_name);
}

// JukeboxRequestHistoryItem
type SetupTrainerRequestDataFn = extern "C" fn(this: *mut Il2CppObject, musicId: *mut Il2CppObject, name: *mut Il2CppString, requestTime: *mut Il2CppObject);
extern "C" fn SetupTrainerRequestData(this: *mut Il2CppObject, musicId: *mut Il2CppObject, name: *mut Il2CppString, requestTime: *mut Il2CppObject) {
    let username = if Hachimi::instance().config.load().hide_usernames {
        "".to_string().to_il2cpp_string();
    } else {
        name
    };
    get_orig_fn!(SetupTrainerRequestData, SetupTrainerRequestDataFn)(this, musicId, username, requestTime);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);
    get_class_or_return!(umamusume, Gallop, JukeboxRequestHistoryItem);

    let SetupRequesterInfo_addr = get_method_addr(JukeboxRequestInfo, c"SetupRequesterInfo", 2);
    new_hook!(SetupRequesterInfo_addr, SetupRequesterInfo);

    let SetupTrainerRequestData_addr = get_method_addr(JukeboxRequestHistoryItem, c"SetupTrainerRequestData", 3);
    new_hook!(SetupTrainerRequestData_addr, SetupTrainerRequestData);
}
