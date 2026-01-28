// public static BgSeason GetSeasonForHome(DateTime dateTime) { }

// TimeUtil
use serde::{Deserialize, Serialize};
use crate::il2cpp::{symbols::get_method_addr, types::*};

#[derive(Default, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[repr(i32)]
pub enum BgSeason {
    #[default] None = 0,
    Spring = 1,
    Summer = 2,
    Fall = 3,
    Winter = 4,
    CherryBlossom = 5
}

// type GetSeasonForHomeFn = extern "C" fn(this: *mut Il2CppObject, dateTime: *mut Il2CppObject) -> BgSeason;
extern "C" fn GetSeasonForHome(this: *mut Il2CppObject, dateTime: *mut Il2CppObject) -> BgSeason {
    // get_orig_fn!(GetSeasonForHome, GetSeasonForHomeFn)(this, dateTime);
    BgSeason::CherryBlossom
}
    
pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TimeUtil);
    
    let GetSeasonForHome_addr = get_method_addr(TimeUtil, c"GetSeasonForHome", 1);
    new_hook!(GetSeasonForHome_addr, GetSeasonForHome);
}
