use crate::il2cpp::{symbols::get_method_addr, types::*};

def_method_wrapper_fn!(IsUniqueSkill, IS_UNIQUE_SKILL_ADDR, bool, this: *mut Il2CppObject);

pub fn init(MasterSkillData: *mut Il2CppClass) {
    find_nested_class_or_return!(MasterSkillData, SkillData);

    unsafe {
        IS_UNIQUE_SKILL_ADDR = get_method_addr(SkillData, c"IsUniqueSkill", 0);
    }
}
