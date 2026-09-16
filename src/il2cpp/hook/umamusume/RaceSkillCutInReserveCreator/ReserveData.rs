use crate::il2cpp::{symbols::get_field_from_name, types::*};

def_field_object_accessors!(get get_ReservedCutInList, RESERVEDCUTINLIST_FIELD, Il2CppObject);

pub fn init(RaceSkillCutInReserveCreator: *mut Il2CppClass) {
    find_nested_class_or_return!(RaceSkillCutInReserveCreator, ReserveData);

    unsafe {
        RESERVEDCUTINLIST_FIELD = get_field_from_name(ReserveData, c"ReservedCutInList");
    }
}
