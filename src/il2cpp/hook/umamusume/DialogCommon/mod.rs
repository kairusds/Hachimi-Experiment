use crate::il2cpp::{
    api::{il2cpp_object_new, il2cpp_runtime_object_init},
    symbols::get_method_addr,
    types::*
};

pub mod Data;

static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn new() -> *mut Il2CppObject {
    let object = il2cpp_object_new(class());
    il2cpp_runtime_object_init(object);
    object
}

static mut INITIALIZE_ADDR: usize = 0;
impl_addr_wrapper_fn!(Initialize, INITIALIZE_ADDR, (), this: *mut Il2CppObject, inData: *mut Il2CppObject);

static mut OPEN_ADDR: usize = 0;
impl_addr_wrapper_fn!(Open, OPEN_ADDR, (), this: *mut Il2CppObject, onComplete: *mut Il2CppDelegate);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCommon);
    Data::init(DialogCommon);

    unsafe {
        CLASS = DialogCommon;
        INITIALIZE_ADDR = get_method_addr(DialogCommon, c"Initialize", 1);
        OPEN_ADDR = get_method_addr(DialogCommon, c"Open", 1);
    }
}