use crate::il2cpp::{symbols::get_method_addr, types::*};

def_method_wrapper_fn!(get_RaceHorseList, GET_RACEHORSELIST_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(TempData: *mut Il2CppClass) {
    find_nested_class_or_return!(TempData, SingleModeTempData);

    unsafe {
        GET_RACEHORSELIST_ADDR = get_method_addr(SingleModeTempData, c"get_RaceHorseList", 0);
    }
}
