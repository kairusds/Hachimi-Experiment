use crate::il2cpp::{symbols::{get_method_addr, get_field_from_name, set_field_object_value}, types::*};

// type get_IsLandscapeModeFn = extern "C" fn(this: *mut Il2CppObject) -> bool;
extern "C" fn get_IsLandscapeMode(this: *mut Il2CppObject) -> bool {
    true
}

// type get_BgSeasonFn = extern "C" fn(this: *mut Il2CppObject) -> i32;
extern "C" fn get_BgSeason(this: *mut Il2CppObject) -> i32 {
    5 // BgSeason.CherryBlossom
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", HomeViewController);

    let get_IsLandscapeMode_addr = get_method_addr(HomeViewController, c"get_IsLandscapeMode", 0);
    let get_BgSeason_addr = get_method_addr(HomeViewController, c"get_BgSeason", 0);
    
    new_hook!(get_IsLandscapeMode_addr, get_IsLandscapeMode);
    new_hook!(get_BgSeason_addr, get_BgSeason);
}

