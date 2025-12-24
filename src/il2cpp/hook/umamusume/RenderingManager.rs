use crate::{core::Hachimi, il2cpp::{symbols::get_method_addr, types::*}};

use super::GraphicSettings::MsaaQuality;

type get_IsOverrideAntiAliasingLevelFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
pub extern "C" fn get_IsOverrideAntiAliasingLevel(this: *mut Il2CppObject) -> bool {
    if Hachimi::instance().config.load().msaa != MsaaQuality::Disabled {
        return true;
    }
    get_orig_fn!(get_IsOverrideAntiAliasingLevel, get_IsOverrideAntiAliasingLevelFn)(this)
}

type get_OverrideAntiAliasingLevelFn = extern "C" fn(this: *mut Il2CppObject) -> i32;
pub extern "C" fn get_OverrideAntiAliasingLevel(this: *mut Il2CppObject) -> i32 {
    if Hachimi::instance().config.load().msaa != MsaaQuality::Disabled {
        return 0; // 0 AntiAliasLevel.Auto
    }
    get_orig_fn!(get_OverrideAntiAliasingLevel, get_OverrideAntiAliasingLevelFn)(this)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop.RenderPipeline", RenderingManager);

    let get_IsOverrideAntiAliasingLevel_addr = get_method_addr(RenderingManager, c"get_IsOverrideAntiAliasingLevel", 0);
    // let get_OverrideAntiAliasingLevel_addr = get_method_addr(RenderingManager, c"get_OverrideAntiAliasingLevel", 0);
    
    new_hook!(get_IsOverrideAntiAliasingLevel_addr, get_IsOverrideAntiAliasingLevel);
    // new_hook!(get_OverrideAntiAliasingLevel_addr, get_OverrideAntiAliasingLevel);
}

