use std::ptr::null_mut;

use crate::il2cpp::{
    symbols::{get_class, get_field_from_name, get_field_value},
    types::*,
};

def_field_object_accessors!(get__position, set__position, POSITION_FIELD, Vector3_t);
def_field_object_accessors!(get__rotationOnLane, set__rotationOnLane, ROTATION_ON_LANE_FIELD, Quaternion_t);

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, HorseRaceInfo);

    unsafe {
        POSITION_FIELD = get_field_from_name(HorseRaceInfo, c"_position");
        ROTATION_ON_LANE_FIELD = get_field_from_name(HorseRaceInfo, c"_rotationOnLane");
    }
}
