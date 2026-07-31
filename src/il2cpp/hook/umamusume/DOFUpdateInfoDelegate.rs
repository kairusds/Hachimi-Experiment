use crate::{
    windows::free_camera,
    il2cpp::{
        symbols::{get_class, get_method_addr},
        types::*,
    },
};

use super::PostEffectUpdateInfo_DOF;

type InvokeFn = extern "C" fn(
    this: *mut Il2CppObject,
    update_info: *mut PostEffectUpdateInfo_DOF::PostEffectUpdateInfoDOF,
);
extern "C" fn Invoke(
    this: *mut Il2CppObject,
    update_info: *mut PostEffectUpdateInfo_DOF::PostEffectUpdateInfoDOF,
) {
    free_camera::set_live_active();
    if free_camera::should_remove_camera_effects() {
        PostEffectUpdateInfo_DOF::disable(update_info);
    }

    get_orig_fn!(Invoke, InvokeFn)(this, update_info);
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop.Live.Cutt", DOFUpdateInfoDelegate);

    let Invoke_addr = get_method_addr(DOFUpdateInfoDelegate, c"Invoke", 1);
    new_hook!(Invoke_addr, Invoke);
}
