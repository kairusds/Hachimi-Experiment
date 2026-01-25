use std::ptr::null_mut;

use crate::il2cpp::{symbols::{get_field_from_name, get_field_value, get_method_addr}, types::*};

pub static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();

static mut TAGID_FIELD: *mut FieldInfo = 0 as _;
pub fn getTagId(this: *mut Il2CppObject) -> *mut Il2CppString {
    get_field_value(this, unsafe { TAGID_FIELD })
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, MasterSkillData);
    find_nested_class_or_return!(MasterSkillData, SkillData);

    unsafe {
        CLASS = SkillData;
        TAGID_FIELD = get_field_from_name(SkillData, c"TagId");
    }
}