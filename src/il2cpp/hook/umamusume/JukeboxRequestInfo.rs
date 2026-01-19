use crate::{core::Hachimi, il2cpp::{ext::StringExt, symbols::get_method_addr, types::*}};

type get_RequestUserNameFn = extern "C" fn(this: *mut Il2CppObject) -> *mut Il2CppString;
extern "C" fn get_RequestUserName(this: *mut Il2CppObject) -> Il2CppString {
    "".to_string().to_il2cpp_string()
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, JukeboxRequestInfo);

    let get_RequestUserName_addr = get_method_addr(JukeboxRequestInfo, c"get_RequestUserName", 0);
    new_hook!(get_RequestUserName_addr, get_RequestUserName);
}