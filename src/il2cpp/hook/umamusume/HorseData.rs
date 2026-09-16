use crate::il2cpp::{
    symbols::{get_field_from_name, get_method_addr},
    types::*,
};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

static mut GET_GATE_NO_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_GateNo, GET_GATE_NO_ADDR, i32, this: *mut Il2CppObject);

static mut GET_CHARA_NAME_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_charaName, GET_CHARA_NAME_ADDR, *mut Il2CppString, this: *mut Il2CppObject);

def_field_value_accessors!(get get_horseIndex, HORSEINDEX_FIELD, i32);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HorseData);

    unsafe {
        CLASS = HorseData;
        GET_GATE_NO_ADDR = get_method_addr(HorseData, c"get_GateNo", 0);
        GET_CHARA_NAME_ADDR = get_method_addr(HorseData, c"get_charaName", 0);
        HORSEINDEX_FIELD = get_field_from_name(HorseData, c"horseIndex");
    }
}
