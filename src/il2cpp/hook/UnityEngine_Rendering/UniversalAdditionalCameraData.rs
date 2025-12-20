use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut SET_ANTIALIASING_ADDR: usize = 0;
static mut SET_ANTIALIASING_QUALITY_ADDR: usize = 0;

impl_addr_wrapper_fn!(set_antialiasing, SET_ANTIALIASING_ADDR, (), this: *mut Il2CppObject, value: i32);
impl_addr_wrapper_fn!(set_antialiasingQuality, SET_ANTIALIASING_QUALITY_ADDR, (), this: *mut Il2CppObject, value: i32);

type OnAfterDeserializeFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn OnAfterDeserialize(this: *mut Il2CppObject) {
    get_orig_fn!(OnAfterDeserialize, OnAfterDeserializeFn)(this);

    // 0 None, 1 FXAA, 2 SMAA, 3 TAA
    set_antialiasing(this, 0);
    // 0 Low, 1 Medium, 2 High
    // set_antialiasingQuality(this, 2);
}

pub fn init(Unity_RenderPipelines_Universal_Runtime: *const Il2CppImage) {
    get_class_or_return!(Unity_RenderPipelines_Universal_Runtime, "UnityEngine.Rendering.Universal", UniversalAdditionalCameraData);

    unsafe {
        SET_ANTIALIASING_ADDR = get_method_addr(UniversalAdditionalCameraData, c"set_antialiasing", 1);
        SET_ANTIALIASING_QUALITY_ADDR = get_method_addr(UniversalAdditionalCameraData, c"set_antialiasingQuality", 1);

        let deserialize_addr = get_method_addr(UniversalAdditionalCameraData, c"OnAfterDeserialize", 0);
        new_hook!(deserialize_addr, OnAfterDeserialize);
    }
}
