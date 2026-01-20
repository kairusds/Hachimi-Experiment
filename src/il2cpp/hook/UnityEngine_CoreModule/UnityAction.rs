use crate::il2cpp::{symbols::{create_delegate, get_method_addr}, api::il2cpp_object_new, types::{UnityAction as UnityActionType, Il2CppImage}};

// static mut CTOR_ADDR: usize = 0;
// .ctor(Object object, IntPtr method)
// impl_addr_wrapper_fn!(ctor, CTOR_ADDR, (), this: *mut UnityAction, target: *mut Il2CppObject, method: usize);

// static mut INVOKE_ADDR: usize = 0;
// impl_addr_wrapper_fn!(Invoke, INVOKE_ADDR, (), this: *mut UnityAction);

/*
pub fn new(rust_fn: usize) -> *mut UnityAction {
    unsafe {
        let klass = find_class("UnityEngine.Events", "UnityAction");
        let action_obj = il2cpp_object_new(klass) as *mut UnityAction;
        ctor(action_obj, std::ptr::null_mut(), rust_fn);
        action_obj
    }
} */

pub fn new_via_symbols<T>(rust_fn: T) -> *mut UnityActionType {
    unsafe {
        let transmuted_fn: fn() = std::mem::transmute_copy(&rust_fn);
        create_delegate(UnityAction, 0, transmuted_fn)
            .map(|d| d as *mut UnityAction)
            .expect("Failed to create UnityAction delegate")
    }
}

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, "UnityEngine.Events", UnityAction);
    /*
    unsafe {
        CTOR_ADDR = get_method_addr(UnityAction, c".ctor", 2);
        INVOKE_ADDR = get_method_addr(UnityAction, c"Invoke", 0);
    } */
}
