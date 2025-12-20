use std::os::raw::c_void;
use crate::il2cpp::{symbols::get_method_addr, types::*};

static mut SET_RENDER_SCALE_ADDR: usize = 0;
static mut SET_MSAA_SAMPLE_COUNT_ADDR: usize = 0;

impl_addr_wrapper_fn!(set_renderScale, SET_RENDER_SCALE_ADDR, (), this: *mut c_void, value: f32);
impl_addr_wrapper_fn!(set_msaaSampleCount, SET_MSAA_SAMPLE_COUNT_ADDR, (), this: *mut c_void, value: i32);

type OnAfterDeserializeFn = extern "C" fn(this: *mut c_void);
extern "C" fn OnAfterDeserialize(this: *mut c_void) {
    get_orig_fn!(OnAfterDeserialize, OnAfterDeserializeFn)(this);

    // 1.2 1080p, 1.5 1440p, 2.0 4k
    set_renderScale(this, 2.0); 
    set_msaaSampleCount(this, 8); 
 
    info!("Hachimi: URP Asset Overridden - Scale: 2, MSAA: 8x");
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
