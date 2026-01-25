use std::ptr::null_mut;

use crate::il2cpp::{symbols::{get_field_from_name, get_field_value, get_method_addr}, types::*};

// pub static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();

static mut SKILLID_FIELD: *mut FieldInfo = 0 as _;
pub fn get_SkillId(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { SKILLID_FIELD })
}

pub fn init(MasterSkillUpgradeDescription: *mut Il2CppClass) {
    find_nested_class_or_return!(MasterSkillUpgradeDescription, SkillUpgradeDescription);

    unsafe {
        // CLASS = SkillUpgradeDescription;
        SKILLID_FIELD = get_field_from_name(SkillUpgradeDescription, c"SkillId");
    }
}