use crate::{
    il2cpp::{
        hook::{
            UnityEngine_CoreModule::{Component, GameObject, RectTransform, Transform},
            UnityEngine_UI::{LayoutElement, LayoutRebuilder, Text}
        },
        symbols::{get_field_from_name, get_field_object_value, get_method_addr},
        types::*
    }
};
use super::PartsSingleModeSkillListItem;

// private PartsSingleModeSkillListItem _partsSingleModeSkillListItem;
static mut _PARTSSINGLEMODESKILLLISTITEM_FIELD: *mut FieldInfo = 0 as _;
pub fn get__partsSingleModeSkillListItem(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _PARTSSINGLEMODESKILLLISTITEM_FIELD })
}

// private Void Setup(SkillData skillData, Boolean isDrawNeedSkillPoint, ShowHintLvUpParam showHintLvUpParam, Int32 skillUpgradeCardId, Boolean isSingleMode, Boolean isDisplayUpgradeSkill) { }
type SetupFn = extern "C" fn(
    this: *mut Il2CppObject, // DialogCharacterSimpleSkillDetail
    skillData: *mut Il2CppObject,
    isDrawNeedSkillPoint: bool,
    showHintLvUpParam: *mut Il2CppObject,
    skillUpgradeCardId: i32,
    isSingleMode: bool,
    isDisplayUpgradeSkill: bool
);
extern "C" fn Setup(
    this: *mut Il2CppObject, // DialogCharacterSimpleSkillDetail
    skillData: *mut Il2CppObject,
    isDrawNeedSkillPoint: bool,
    showHintLvUpParam: *mut Il2CppObject,
    skillUpgradeCardId: i32,
    isSingleMode: bool,
    isDisplayUpgradeSkill: bool
) {
    get_orig_fn!(Setup, SetupFn)(this, skillData, isDrawNeedSkillPoint, showHintLvUpParam, skillUpgradeCardId, isSingleMode, isDisplayUpgradeSkill);

    let skill_item = get__partsSingleModeSkillListItem(this);
    if skill_item.is_null() {
        info!("DialogCharacterSimpleSkillDetail.Setup skill_item is null");
        return;
    }

    let desc_text = PartsSingleModeSkillListItem::get__descText(skill_item);
    info!("desc_text: {:p}", desc_text);
    let bg_btn = PartsSingleModeSkillListItem::get__bgButton(skill_item);
    info!("bg_btn: {:p}", bg_btn);
    let bg_obj = Component::get_gameObject(bg_btn);

    if !desc_text.is_null() && !bg_obj.is_null() {
        Text::set_horizontalOverflow(desc_text, 0); // wrap
        Text::set_verticalOverflow(desc_text, 1); // overflow
        let needed_height = Text::get_preferredHeight(desc_text);
        info!("needed_height {}", needed_height);

        let mut layout_element = GameObject::GetComponent(bg_obj, LayoutElement::type_object());
        if layout_element.is_null() {
            info!("LayoutElement for _bgButton empty, building new one...");
            layout_element = GameObject::AddComponent(bg_obj, LayoutElement::type_object());
        }

        // let image = GameObject::GetComponent(bg_obj, ImageCommon_Class);
        // Image::set_type(image, 1);

        let final_height = needed_height + 150.0;
        info!("final_height; {}", final_height);
        LayoutElement::set_minHeight(layout_element, final_height);
        LayoutElement::set_flexibleHeight(layout_element, 0.0);

        let this_obj = Component::get_gameObject(this);
        if this.is_null() {
            info!("this_obj is NULL");
            return;
        }

        let mut this_layout = GameObject::GetComponent(this_obj, LayoutElement::type_object());
        if this_layout.is_null() {
            this_layout = GameObject::AddComponent(this_obj, LayoutElement::type_object());
        }
        LayoutElement::set_minHeight(this_layout, final_height);
        LayoutElement::set_flexibleHeight(this_layout, 0.0);

        let inner_rect = GameObject::GetComponent(this_obj, RectTransform::type_object());
        if inner_rect.is_null() {
            info!("inner_rect is null");
            return;
        }
        LayoutRebuilder::ForceRebuildLayoutImmediate(inner_rect);

        let parent_transform = Transform::get_parent(inner_rect);
        if parent_transform.is_null() {
            info!("parent_transform is null");
            return;
        }
        let parent_game_obj = Component::get_gameObject(parent_transform);
        let parent_rect = GameObject::GetComponent(parent_game_obj, RectTransform::type_object());
        if parent_rect.is_null() {
            info!("parent_rect is null");
            return;
        }
        LayoutRebuilder::ForceRebuildLayoutImmediate(parent_rect);

        let grand_parent_transform = Transform::get_parent(parent_rect);
        if grand_parent_transform.is_null() {
            info!("grand_parent_transform is null");
            return;
        }
        let gp_game_obj = Component::get_gameObject(grand_parent_transform);
        let gp_rect = GameObject::GetComponent(gp_game_obj, RectTransform::type_object());
        if gp_rect.is_null() {
            info!("gp_rect is null");
            return;
        }
        LayoutRebuilder::ForceRebuildLayoutImmediate(gp_rect);
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCharacterSimpleSkillDetail);

    let Setup_addr = get_method_addr(DialogCharacterSimpleSkillDetail, c"Setup", 6);
    new_hook!(Setup_addr, Setup);

    unsafe {
        _PARTSSINGLEMODESKILLLISTITEM_FIELD = get_field_from_name(DialogCharacterSimpleSkillDetail, c"_partsSingleModeSkillListItem");
    }
}
