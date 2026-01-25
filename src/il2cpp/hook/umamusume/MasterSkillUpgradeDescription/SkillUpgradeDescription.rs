use std::ptr::null_mut;

use crate::il2cpp::{symbols::{get_field_from_name, get_field_value, get_method_addr}, types::*};

// pub static mut CLASS: *mut Il2CppClass = std::ptr::null_mut();

static mut ID_FIELD: *mut FieldInfo = 0 as _;
pub fn get_Id(this: *mut Il2CppObject) -> i32 {
    get_field_value(this, unsafe { ID_FIELD })
}

pub fn init(MasterSkillUpgradeDescription: *mut Il2CppClass) {
    find_nested_class_or_return!(MasterSkillUpgradeDescription, SkillUpgradeDescription);

    unsafe {
        // CLASS = SkillUpgradeDescription;
        ID_FIELD = get_field_from_name(SkillUpgradeDescription, c"Id");
        if ID_FIELD.is_null() {
            error!("Failed to find field Id in SkillUpgradeDescription");
        }
    }
}