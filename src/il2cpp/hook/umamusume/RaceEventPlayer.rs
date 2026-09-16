use crate::{
    core::Hachimi,
    il2cpp::{
        api::il2cpp_class_is_assignable_from,
        ext::Il2CppObjectExt,
        symbols::get_method_addr,
        types::*
    }
};

use super::{HorseRaceInfo, MasterSkillData, SkillBase};

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn is_event_player(obj: *mut Il2CppObject) -> bool {
    if class().is_null() || obj.is_null() {
        return false;
    }

    let obj_class = unsafe { (*obj).klass() };
    !obj_class.is_null() && il2cpp_class_is_assignable_from(class(), obj_class)
}

def_method_wrapper_fn!(ChangeLastEventIndexByTime, CHANGE_LAST_EVENT_INDEX_BY_TIME_ADDR, (), this: *mut Il2CppObject, time: f32);
def_method_wrapper_fn!(
    GetSkillEventParam,
    GET_SKILL_EVENT_PARAM_ADDR,
    (),
    sim_ev_data: *mut Il2CppObject,
    horse_idx: *mut i32,
    skill_id: *mut i32,
    detail_index: *mut i32,
    time_int: *mut i32,
    target_flags: *mut i32,
    caller_skill_id: *mut i32,
    activate_type: *mut i32,
    ability_value_status: *mut i32,
    ability_time_status: *mut i32
);

type IsPlayableCutInFn = extern "C" fn(this: *mut Il2CppObject, info: *mut Il2CppObject, skill: *mut Il2CppObject, activate_type: i32) -> bool;
pub extern "C" fn IsPlayableCutIn(this: *mut Il2CppObject, info: *mut Il2CppObject, skill: *mut Il2CppObject, activate_type: i32) -> bool {
    if get_orig_fn!(IsPlayableCutIn, IsPlayableCutInFn)(this, info, skill, activate_type) {
        return true;
    }
    if !Hachimi::instance().config.load().race_play_others_cutins {
        return false;
    }
    if info.is_null() || skill.is_null() {
        return false;
    }
    if HorseRaceInfo::IsPlayerHorse(info) {
        return false;
    }
    let skill_master = SkillBase::get_SkillMaster(skill);
    if skill_master.is_null() {
        return false;
    }
    MasterSkillData::SkillData::IsUniqueSkill(skill_master)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, RaceEventPlayer);

    let IsPlayableCutIn_addr = get_method_addr(RaceEventPlayer, c"IsPlayableCutIn", 3);
    new_hook!(IsPlayableCutIn_addr, IsPlayableCutIn);

    unsafe {
        CLASS = RaceEventPlayer;
        CHANGE_LAST_EVENT_INDEX_BY_TIME_ADDR = get_method_addr(RaceEventPlayer, c"ChangeLastEventIndexByTime", 1);
        GET_SKILL_EVENT_PARAM_ADDR = get_method_addr(RaceEventPlayer, c"GetSkillEventParam", 10);
    }
}
