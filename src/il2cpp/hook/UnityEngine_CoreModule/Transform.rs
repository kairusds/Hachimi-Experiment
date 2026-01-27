use crate::{
    il2cpp::{
        symbols::get_method_addr,
        types::*
    }
};

// public Transform get_parent() { }
static mut GET_PARENT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_parent, GET_PARENT_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, Transform);

    unsafe {
        GET_PARENT_ADDR = get_method_addr(Transform, c"get_parent", 0);
    }
}
