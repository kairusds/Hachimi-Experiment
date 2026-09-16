use crate::il2cpp::{symbols::{get_field_from_name, SingletonLike}, types::*};

pub mod JukeboxSetListPlayingData;
pub mod SingleModeTempData;

static mut CLASS: *mut Il2CppClass = 0 as _;
pub fn class() -> *mut Il2CppClass {
    unsafe { CLASS }
}

pub fn instance() -> *mut Il2CppObject {
    let Some(singleton) = SingletonLike::new(class()) else {
        return 0 as _;
    };
    singleton.instance()
}

def_field_object_accessors!(get get_SingleModeData, SINGLEMODEDATA_FIELD, Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, TempData);

    unsafe {
        CLASS = TempData;
        SINGLEMODEDATA_FIELD = get_field_from_name(TempData, c"SingleModeData");
    }

    JukeboxSetListPlayingData::init(TempData);
    SingleModeTempData::init(TempData);
}
