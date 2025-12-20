use crate::il2cpp::{symbols::get_method_addr, types::*};

// type get_allowMSAAFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
extern "C" fn get_allowMSAA(this: *mut Il2CppObject) -> bool {
    true
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Camera);

    unsafe {
        let get_allowMSAA_addr = get_method_addr(Camera, c"get_allowMSAA", 0);
        new_hook!(get_allowMSAA_addr, get_allowMSAA);
    }
}
