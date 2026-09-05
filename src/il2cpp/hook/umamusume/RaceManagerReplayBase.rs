use crate::il2cpp::{
    symbols::get_method_addr,
    types::*
};

def_method_wrapper_fn!(ForceSetRaceTime, FORCE_SET_RACE_TIME_ADDR, (), this: *mut Il2CppObject, time: f32, isCheckEvent: bool);
def_method_wrapper_fn!(IsPaused, IS_PAUSED_ADDR, bool, this: *mut Il2CppObject);
def_method_wrapper_fn!(PauseRace, PAUSE_RACE_ADDR, (), this: *mut Il2CppObject);
def_method_wrapper_fn!(ResumeRace, RESUME_RACE_ADDR, (), this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, RaceManagerReplayBase);

    unsafe {
        FORCE_SET_RACE_TIME_ADDR = get_method_addr(RaceManagerReplayBase, c"ForceSetRaceTime", 2);
        IS_PAUSED_ADDR = get_method_addr(RaceManagerReplayBase, c"IsPaused", 0);
        PAUSE_RACE_ADDR = get_method_addr(RaceManagerReplayBase, c"PauseRace", 0);
        RESUME_RACE_ADDR = get_method_addr(RaceManagerReplayBase, c"ResumeRace", 0);
    }
}
