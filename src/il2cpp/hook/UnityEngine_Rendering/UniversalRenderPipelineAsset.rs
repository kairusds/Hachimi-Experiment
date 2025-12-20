use crate::il2cpp::{symbols::get_method_addr, types::*};

/*
pub extern "C" fn get_renderScale(this: *mut Il2CppObject) -> f32 {
    2.0
}

pub extern "C" fn get_msaaSampleCount(this: *mut Il2CppObject) -> i32 {
    8
}

pub fn init(Unity_RenderPipelines_Universal_Runtime: *const Il2CppImage) {
    get_class_or_return!(Unity_RenderPipelines_Universal_Runtime, "UnityEngine.Rendering.Universal", UniversalRenderPipelineAsset);

    unsafe {
        let get_scale_addr = get_method_addr(UniversalRenderPipelineAsset, c"get_renderScale", 0);
        let get_msaa_addr = get_method_addr(UniversalRenderPipelineAsset, c"get_msaaSampleCount", 0);

        new_hook!(get_scale_addr, get_renderScale);
        new_hook!(get_msaa_addr, get_msaaSampleCount);
    }
}
*/

static mut GET_RENDER_SCALE_ADDR: usize = 0;
static mut SET_RENDER_SCALE_ADDR: usize = 0;
static mut SET_MSAA_SAMPLE_COUNT_ADDR: usize = 0;

impl_addr_wrapper_fn!(get_renderScale, GET_RENDER_SCALE_ADDR, (), this: *mut Il2CppObject, value: f32);
impl_addr_wrapper_fn!(set_renderScale, SET_RENDER_SCALE_ADDR, (), this: *mut Il2CppObject, value: f32);
impl_addr_wrapper_fn!(set_msaaSampleCount, SET_MSAA_SAMPLE_COUNT_ADDR, (), this: *mut Il2CppObject, value: i32);

type OnAfterDeserializeFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn OnAfterDeserialize(this: *mut Il2CppObject) {
    get_orig_fn!(OnAfterDeserialize, OnAfterDeserializeFn)(this);

    // 1.2 1080p, 1.5 1440p, 2.0 4k
    set_renderScale(this, 2.0); 
    let current = get_renderScale(this);
    info!("Hachimi: RenderScale set to {}, actual value is {}", 2.0, current);
    set_msaaSampleCount(this, 4);
}

pub fn init(Unity_RenderPipelines_Universal_Runtime: *const Il2CppImage) {
    get_class_or_return!(Unity_RenderPipelines_Universal_Runtime, "UnityEngine.Rendering.Universal", UniversalRenderPipelineAsset);

    unsafe {
        SET_RENDER_SCALE_ADDR = get_method_addr(UniversalRenderPipelineAsset, c"set_renderScale", 1);
        SET_MSAA_SAMPLE_COUNT_ADDR = get_method_addr(UniversalRenderPipelineAsset, c"set_msaaSampleCount", 1);

        let deserialize_addr = get_method_addr(UniversalRenderPipelineAsset, c"OnAfterDeserialize", 0);
        new_hook!(deserialize_addr, OnAfterDeserialize);
    }
}
