use std::ptr::null_mut;
use crate::{il2cpp::{symbols::{get_method_addr, SingletonLike}, types::*}};

static mut CLASS: *mut Il2CppClass = null_mut();
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = SingletonLike::new(class()) else {
        return null_mut();
    };
    singleton.instance()
}

// public static DialogCommon PushDialog(Data data) { }
static mut PUSHDIALOG_ADDR: usize = 0;
impl_addr_wrapper_fn!(PushDialog, PUSHDIALOG_ADDR, (), data: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogManager);

    unsafe {
        CLASS = DialogManager;
        PUSHDIALOG_ADDR = get_method_addr(DialogManager, c"PushDialog", 1);
    }
}
