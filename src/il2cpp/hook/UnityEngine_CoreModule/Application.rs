use std::sync::atomic;

use crate::{core::Hachimi, il2cpp::{api::il2cpp_resolve_icall, types::*}};

type SetTargetFrameRateFn = extern "C" fn(value: i32);
pub extern "C" fn set_targetFrameRate(mut value: i32) {
    let target_fps = Hachimi::instance().target_fps.load(atomic::Ordering::Relaxed);
    if target_fps != -1 {
        value = target_fps;
    }
    get_orig_fn!(set_targetFrameRate, SetTargetFrameRateFn)(value);
}

type GetRunInBackgroundFn = extern "C" fn() -> bool;
pub extern "C" fn get_runInBackground() -> bool {
    true
}

pub fn init(_UnityEngine_CoreModule: *const Il2CppImage) {
    let set_targetFrameRate_addr = il2cpp_resolve_icall(
        c"UnityEngine.Application::set_targetFrameRate(System.Int32)".as_ptr()
    );
    new_hook!(set_targetFrameRate_addr, set_targetFrameRate);

    let get_runInBackground_addr = il2cpp_resolve_icall(
        c"UnityEngine.Application::get_runInBackground()".as_ptr()
    );
    new_hook!(get_runInBackground_addr, get_runInBackground);
}