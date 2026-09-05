use crate::il2cpp::{
    symbols::{get_field_from_name, get_method_addr},
    types::*
};

def_field_object_accessors!(get__simData, set__simData, SIM_DATA_FIELD, Il2CppObject);
def_field_value_accessors!(get__curTime, set__curTime, CUR_TIME_FIELD, f32);

def_method_wrapper_fn!(GetLastFrameTime, GET_LAST_FRAME_TIME_ADDR, f32, this: *mut Il2CppObject);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, RaceSimulateReader);

    unsafe {
        SIM_DATA_FIELD = get_field_from_name(RaceSimulateReader, c"_simData");
        CUR_TIME_FIELD = get_field_from_name(RaceSimulateReader, c"_curTime");
        GET_LAST_FRAME_TIME_ADDR = get_method_addr(RaceSimulateReader, c"GetLastFrameTime", 0);
    }
}
