use crate::{
    core::Hachimi,
    il2cpp::{hook::UnityEngine_UI::Text, sql::TextDataQuery, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}
};

static mut DESCTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__descText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DESCTEXT_FIELD })
}

type UpdateItemFn = extern "C" fn(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, resource_hash: i32);
extern "C" fn UpdateItem(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, resource_hash: i32) {

    let mut mult:f32 = 0.0;
    if resource_hash == TextResourceHash::SkillDesc as i32 {
        let config = &Hachimi::instance().localized_data.load().config;
        mult = config.skill_list_item_desc_line_width_multiplier.unwrap_or(1.0);
        Text::set_horizontalOverflow(get__descText(this), 1);
    }

    TextDataQuery::with_skill_query(mult, || {
        get_orig_fn!(UpdateItem, UpdateItemFn)(this, skill_info, is_plate_effect_enable, resource_hash);
    });
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeSkillListItem);

    let UpdateItem_addr = get_method_addr(PartsSingleModeSkillListItem, c"UpdateItem", 3);

    new_hook!(UpdateItem_addr, UpdateItem);

    unsafe {
        DESCTEXT_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_descText");
    }
}