use std::ptr::null_mut;

use crate::il2cpp::{
    symbols::{get_method_addr, SingletonLike},
    types::*,
};

pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = SingletonLike::new(class()) else {
        return 0 as _;
    };
    singleton.instance()
}

static mut REFRESH_ALL_ADDR: usize = 0;
impl_addr_wrapper_fn!(RefreshAll, REFRESH_ALL_ADDR, (), this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TapEffectController);

    unsafe {
        REFRESH_ALL_ADDR = get_method_addr(TapEffectController, c"RefreshAll", 0);
    }
}
