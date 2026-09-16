use crate::{
    core::{Hachimi, game::Region},
    il2cpp::{
        symbols::get_method_addr,
        types::*
    }
};

static mut GET_SKILL_MASTER_ID_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_SkillMasterId, GET_SKILL_MASTER_ID_ADDR, i32, this: *mut Il2CppObject);

static mut GET_SKILL_MASTER_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_SkillMaster, GET_SKILL_MASTER_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, SkillBase);

    unsafe {
        GET_SKILL_MASTER_ADDR = get_method_addr(SkillBase, c"get_SkillMaster", 0);
        if matches!(Hachimi::instance().game.region, Region::Japan | Region::Global) {
            GET_SKILL_MASTER_ID_ADDR = get_method_addr(SkillBase, c"get_SkillMasterId", 0);
        }
    }
}
