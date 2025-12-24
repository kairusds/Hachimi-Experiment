use crate::{core::Hachimi, il2cpp::{symbols::get_method_addr, types::*}};

type get_IsOverrideAntiAliasingLevelFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
pub extern "C" fn get_IsOverrideAntiAliasingLevel(this: *mut Il2CppObject) -> bool {
    if Hachimi::instance().config.load().msaa != super::GraphicSettings::MsaaQuality::Disabled {
        return true;
    }
    get_orig_fn!(get_IsOverrideAntiAliasingLevel, get_IsOverrideAntiAliasingLevelFn)(this)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop.RenderPipeline", RenderingManager);

    let get_IsOverrideAntiAliasingLevel_addr = get_method_addr(RenderingManager, c"get_IsOverrideAntiAliasingLevel", 0);
    
    new_hook!(get_IsOverrideAntiAliasingLevel_addr, get_IsOverrideAntiAliasingLevel);
}

