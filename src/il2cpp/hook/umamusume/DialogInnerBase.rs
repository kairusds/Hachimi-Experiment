use crate::il2cpp::{symbols::get_method_addr, types::*};

// public DialogCommonBase GetDialog() { }
static mut GETDIALOG_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetDialog, GETDIALOG_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogInnerBase);

    unsafe {
        GETDIALOG_ADDR = get_method_addr(DialogInnerBase, c"GetDialog", 0);
    }
}
