use crate::il2cpp::{symbols::get_method_addr, types::*};

// type get_IsOverrideAntiAliasingLevelFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
pub extern "C" fn get_IsOverrideAntiAliasingLevel(_this: *mut Il2CppObject) -> bool {
    true
}

// type get_OverrideAntiAliasingLevelFn = extern "C" fn(this: *mut Il2CppObject) -> i32;
pub extern "C" fn get_OverrideAntiAliasingLevel(_this: *mut Il2CppObject) -> i32 {
    0 // 0 Auto
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop.RenderPipeline", RenderingManager);

    let get_IsOverrideAntiAliasingLevel_addr = get_method_addr(RenderingManager, c"get_IsOverrideAntiAliasingLevel", 0);
    let get_OverrideAntiAliasingLevel_addr = get_method_addr(RenderingManager, c"get_OverrideAntiAliasingLevel", 0);
    
    new_hook!(get_IsOverrideAntiAliasingLevel_addr, get_IsOverrideAntiAliasingLevel);
    new_hook!(get_OverrideAntiAliasingLevel_addr, get_OverrideAntiAliasingLevel);
}

