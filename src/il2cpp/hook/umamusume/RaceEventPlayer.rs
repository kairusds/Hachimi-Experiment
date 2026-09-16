use crate::{
    core::{game::Region, Hachimi},
    il2cpp::{
        api::il2cpp_class_is_assignable_from,
        ext::Il2CppObjectExt,
        symbols::get_method_addr,
        types::*
    }
};

use super::{HorseRaceInfo, MasterSkillData, RaceSkillCutInReserveCreator, SkillBase};

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

static mut PENDING_CUTIN_HORSE: i32 = -1;

type GetSkillEventParamFn = extern "C" fn(
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
pub extern "C" fn GetSkillEventParam(
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
) {
    get_orig_fn!(GetSkillEventParam, GetSkillEventParamFn)(
        sim_ev_data,
        horse_idx,
        skill_id,
        detail_index,
        time_int,
        target_flags,
        caller_skill_id,
        activate_type,
        ability_value_status,
        ability_time_status
    );
    track_pending_cutin_horse(sim_ev_data, horse_idx, skill_id);
}

type GetSkillEventParamOtherFn = extern "C" fn(
    sim_ev_data: *mut Il2CppObject,
    horse_idx: *mut i32,
    skill_id: *mut i32,
    detail_index: *mut i32,
    time_int: *mut i32,
    target_flags: *mut i32,
    activate_type: *mut i32
);
pub extern "C" fn GetSkillEventParamOther(
    sim_ev_data: *mut Il2CppObject,
    horse_idx: *mut i32,
    skill_id: *mut i32,
    detail_index: *mut i32,
    time_int: *mut i32,
    target_flags: *mut i32,
    activate_type: *mut i32
) {
    get_orig_fn!(GetSkillEventParamOther, GetSkillEventParamOtherFn)(
        sim_ev_data,
        horse_idx,
        skill_id,
        detail_index,
        time_int,
        target_flags,
        activate_type
    );
    track_pending_cutin_horse(sim_ev_data, horse_idx, skill_id);
}

fn track_pending_cutin_horse(sim_ev_data: *mut Il2CppObject, horse_idx: *mut i32, skill_id: *mut i32) {
    unsafe {
        PENDING_CUTIN_HORSE = -1;
        if Hachimi::instance().config.load().race_play_others_cutins
            && !sim_ev_data.is_null()
            && RaceSkillCutInReserveCreator::widened_cutin_scheduled(*horse_idx, *skill_id) {
            PENDING_CUTIN_HORSE = *horse_idx;
        }
    }
}

// might not be needed for showing others cutins
type IsShowSkillEnemyFn = extern "C" fn(this: *mut Il2CppObject, team_id: i32, horse_index: i32, is_mob: bool) -> bool;
pub extern "C" fn IsShowSkillEnemy(this: *mut Il2CppObject, team_id: i32, horse_index: i32, is_mob: bool) -> bool {
    if Hachimi::instance().config.load().race_play_others_cutins && unsafe { PENDING_CUTIN_HORSE } == horse_index {
        return true;
    }

    get_orig_fn!(IsShowSkillEnemy, IsShowSkillEnemyFn)(this, team_id, horse_index, is_mob)
}

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

    let IsShowSkillEnemy_addr = get_method_addr(RaceEventPlayer, c"IsShowSkillEnemy", 3);
    new_hook!(IsShowSkillEnemy_addr, IsShowSkillEnemy);

    if Hachimi::instance().game.region == Region::Japan {
        let GetSkillEventParam_addr = get_method_addr(RaceEventPlayer, c"GetSkillEventParam", 10);
        new_hook!(GetSkillEventParam_addr, GetSkillEventParam);
    } else {
        let GetSkillEventParam_addr = get_method_addr(RaceEventPlayer, c"GetSkillEventParam", 6);
        new_hook!(GetSkillEventParam_addr, GetSkillEventParamOther);
    }

    unsafe {
        CLASS = RaceEventPlayer;
        CHANGE_LAST_EVENT_INDEX_BY_TIME_ADDR = get_method_addr(RaceEventPlayer, c"ChangeLastEventIndexByTime", 1);
    }
}
