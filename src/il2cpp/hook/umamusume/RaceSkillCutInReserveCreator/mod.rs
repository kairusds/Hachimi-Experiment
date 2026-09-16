use crate::{
    core::Hachimi,
    il2cpp::{
        symbols::{get_method_addr, Array, IList},
        types::*
    }
};

use super::{HorseData, RaceDefine::RaceType, TempData::{self, SingleModeTempData}};

pub mod ReserveData;
pub mod ReservedCutIn;

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

def_method_wrapper_fn!(get_Reserve, GET_RESERVE_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

static mut WIDENED_CUTIN_KEYS: [(i32, i32); 64] = [(-1, -1); 64];
static mut WIDENED_CUTIN_KEY_COUNT: usize = 0;

pub fn widened_cutin_scheduled(horse_idx: i32, skill_id: i32) -> bool {
    unsafe {
        for key in &WIDENED_CUTIN_KEYS[..WIDENED_CUTIN_KEY_COUNT] {
            if key.0 == horse_idx && key.1 == skill_id {
                return true;
            }
        }
    }
    false
}

type CreatePlayReservedFn = extern "C" fn(
    this: *mut Il2CppObject,
    cut_in_playable_chara_array: *mut Il2CppArray,
    race_type: i32,
    event_data_list: *mut Il2CppObject,
    boot_mode: i32,
);
pub extern "C" fn CreatePlayReserved(
    this: *mut Il2CppObject,
    cut_in_playable_chara_array: *mut Il2CppArray,
    race_type: i32,
    event_data_list: *mut Il2CppObject,
    _boot_mode: i32,
) {
    let orig_fn = get_orig_fn!(CreatePlayReserved, CreatePlayReservedFn);
    unsafe {
        WIDENED_CUTIN_KEY_COUNT = 0;
    }

    let widened = if Hachimi::instance().config.load().race_play_others_cutins
        && (race_type == RaceType::Single
            || race_type == RaceType::SingleModeScenarioTeamRace) {
        widen_playable_array(cut_in_playable_chara_array)
    } else {
        None
    };

    match widened {
        Some(array) => {
            orig_fn(this, array, race_type, event_data_list, _boot_mode);
            snapshot_widened_keys(this, cut_in_playable_chara_array);
        }
        None => orig_fn(this, cut_in_playable_chara_array, race_type, event_data_list, _boot_mode),
    }
}

fn snapshot_widened_keys(this: *mut Il2CppObject, playable: *mut Il2CppArray) {
    let playable_arr: Array<*mut Il2CppObject> = Array::from(playable);
    let playable_len = playable_arr.len();

    let Some(reserves) = IList::<*mut Il2CppObject>::new(ReserveData::get_ReservedCutInList(
        get_Reserve(this)
    )) else {
        return;
    };
    for reserve in reserves.iter() {
        if reserve.is_null() {
            continue;
        }

        let horse_index = ReservedCutIn::get_HorseIndex(reserve);
        let mut vanilla = false;
        for i in 0..playable_len {
            let horse = unsafe { playable_arr.as_slice()[i] };
            if !horse.is_null() && HorseData::get_horseIndex(horse) == horse_index {
                vanilla = true;
                break;
            }
        }

        if vanilla {
            continue;
        }

        unsafe {
            if WIDENED_CUTIN_KEY_COUNT == WIDENED_CUTIN_KEYS.len() {
                break;
            }

            WIDENED_CUTIN_KEYS[WIDENED_CUTIN_KEY_COUNT] =
                (horse_index, ReservedCutIn::get_SkillId(reserve));
            WIDENED_CUTIN_KEY_COUNT += 1;
        }
    }
}

fn widen_playable_array(playable: *mut Il2CppArray) -> Option<*mut Il2CppArray> {
    let temp_data = TempData::instance();
    if temp_data.is_null() {
        return None;
    }

    let race_horse_list = SingleModeTempData::get_RaceHorseList(TempData::get_SingleModeData(temp_data));
    let Some(race_horses) = IList::<*mut Il2CppObject>::new(race_horse_list) else {
        return None;
    };

    let playable_arr: Array<*mut Il2CppObject> = Array::from(playable);
    let playable_len = playable_arr.len();

    let mut additions: [*mut Il2CppObject; 24] = [std::ptr::null_mut(); 24];
    let mut addition_count = 0usize;

    for horse in race_horses.iter() {
        if horse.is_null() {
            continue;
        }

        let horse_index = HorseData::get_horseIndex(horse);
        let mut already = false;

        for i in 0..playable_len {
            let existing = unsafe { playable_arr.as_slice()[i] };
            if !existing.is_null() && HorseData::get_horseIndex(existing) == horse_index {
                already = true;
                break;
            }
        }

        if already {
            continue;
        }

        let mut duplicate = false;
        for addition in &additions[..addition_count] {
            if HorseData::get_horseIndex(*addition) == horse_index {
                duplicate = true;
                break;
            }
        }

        if !duplicate {
            if addition_count == additions.len() {
                break;
            }
            additions[addition_count] = horse;
            addition_count += 1;
        }
    }

    if addition_count == 0 {
        return None;
    }

    let combined: Array<*mut Il2CppObject> = Array::new(HorseData::class(), playable_len + addition_count);
    unsafe {
        let elements = combined.as_slice();
        elements[..playable_len].copy_from_slice(&playable_arr.as_slice()[..playable_len]);
        elements[playable_len..playable_len + addition_count].copy_from_slice(&additions[..addition_count]);
    }
    Some(combined.into())
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, RaceSkillCutInReserveCreator);

    let CreatePlayReserved_addr = get_method_addr(RaceSkillCutInReserveCreator, c"CreatePlayReserved", 4);
    new_hook!(CreatePlayReserved_addr, CreatePlayReserved);

    unsafe {
        CLASS = RaceSkillCutInReserveCreator;
        GET_RESERVE_ADDR = get_method_addr(RaceSkillCutInReserveCreator, c"get_Reserve", 0);
    }

    ReserveData::init(RaceSkillCutInReserveCreator);
    ReservedCutIn::init(RaceSkillCutInReserveCreator);
}
