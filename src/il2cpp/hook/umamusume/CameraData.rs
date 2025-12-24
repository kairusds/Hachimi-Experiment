use crate::{core::Hachimi, il2cpp::{symbols::{get_method_addr, get_field_from_name, set_field_value}, types::*}};

use super::GraphicSettings::MsaaQuality;

static mut REQUESTANTIALIASING_FIELD: *mut FieldInfo = 0 as _;
pub fn set_RequestAntiAliasing(this: *mut Il2CppObject, value: i32) {
    set_field_value(this, unsafe { REQUESTANTIALIASING_FIELD }, &value);
}

// type get_IsCreateAntialiasTextureFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
// likely culprit for chibis on jukebox disappearing in windows
/*
extern "C" fn get_IsCreateAntialiasTexture(this: *mut Il2CppObject) -> bool {
    true
}
*/

type get_RenderingAntiAliasingFn = extern "C" fn(this: *mut Il2CppObject) -> i32;
extern "C" fn get_RenderingAntiAliasing(this: *mut Il2CppObject) -> i32 {
    let msaa = Hachimi::instance().config.load().msaa;
    if msaa != MsaaQuality::Disabled {
        return msaa;
    }
    get_orig_fn!(get_RenderingAntiAliasing, get_RenderingAntiAliasingFn)(this)
}

static mut UPDATEANTIALIASPARAMETER_ADDR: usize = 0;
impl_addr_wrapper_fn!(UpdateAntiAliasParameter, UPDATEANTIALIASPARAMETER_ADDR, (), this: *mut Il2CppObject);

type InitializeFn = extern "C" fn(this: *mut Il2CppObject,  rendererIndex: i32);
extern "C" fn Initialize(this: *mut Il2CppObject, rendererIndex: i32) {
    get_orig_fn!(Initialize, InitializeFn)(this, rendererIndex);

    if Hachimi::instance().config.load().msaa != MsaaQuality::Disabled {
        // AntiAliasLevel.Auto
        set_RequestAntiAliasing(this, 0);
        UpdateAntiAliasParameter(this);
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop.RenderPipeline", CameraData);

    // let get_IsCreateAntialiasTexture_addr = get_method_addr(CameraData, c"get_IsCreateAntialiasTexture", 0);
    let Initialize_addr = get_method_addr(CameraData, c"Initialize", 1);
    let get_RenderingAntiAliasing_addr = get_method_addr(CameraData, c"get_RenderingAntiAliasing", 0);
    
    // new_hook!(get_IsCreateAntialiasTexture_addr, get_IsCreateAntialiasTexture);
    new_hook!(Initialize_addr, Initialize);
    new_hook!(get_RenderingAntiAliasing_addr, get_RenderingAntiAliasing);

    unsafe {
        REQUESTANTIALIASING_FIELD = get_field_from_name(CameraData, c"RequestAntiAliasing");
        UPDATEANTIALIASPARAMETER_ADDR = get_method_addr(CameraData, c"UpdateAntiAliasParameter", 0);
    }
}

