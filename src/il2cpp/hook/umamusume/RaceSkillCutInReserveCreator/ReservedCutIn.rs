use crate::il2cpp::{symbols::get_field_from_name, types::*};

def_field_value_accessors!(get get_HorseIndex, HORSEINDEX_FIELD, i32);
def_field_value_accessors!(get get_SkillId, SKILLID_FIELD, i32);

pub fn init(RaceSkillCutInReserveCreator: *mut Il2CppClass) {
    find_nested_class_or_return!(RaceSkillCutInReserveCreator, ReservedCutIn);

    unsafe {
        HORSEINDEX_FIELD = get_field_from_name(ReservedCutIn, c"HorseIndex");
        SKILLID_FIELD = get_field_from_name(ReservedCutIn, c"SkillId");
    }
}
