use crate::il2cpp::{hook::UnityEngine_UI::Text, sql::TextDataQuery, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*};

static mut DESCTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__descText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DESCTEXT_FIELD })
}

type UpdateCurrentFn = extern "C" fn(this: *mut Il2CppObject);
extern "C" fn UpdateCurrent(this: *mut Il2CppObject) {
    Text::set_horizontalOverflow(get__descText(this), 1);

    TextDataQuery::with_skill_query(1.0, || {
        get_orig_fn!(UpdateCurrent, UpdateCurrentFn)(this);
    });
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeSkillLearningListItem);

    let UpdateCurrent_addr = get_method_addr(PartsSingleModeSkillLearningListItem, c"UpdateCurrent", 0);

    new_hook!(UpdateCurrent_addr, UpdateCurrent);

    unsafe {
        DESCTEXT_FIELD = get_field_from_name(PartsSingleModeSkillLearningListItem, c"_descriptionText");
    }
}