use crate::{
    il2cpp::{
        hook::{
            umamusume::{DialogCharacterSimpleSkillDetail, DialogCommonBase, PartsSingleModeSkillListItem},
            UnityEngine_CoreModule::{Component, GameObject, RectTransform, Transform},
            UnityEngine_UI::{LayoutElement, LayoutRebuilder, Text}
        },
        symbols::get_method_addr,
        types::*
    }
};

pub mod Data;

type InitializeFn = extern "C" fn(this: *mut Il2CppObject, inData: *mut Il2CppObject);
extern "C" fn Initialize(this: *mut Il2CppObject, inData: *mut Il2CppObject) {
    get_orig_fn!(Initialize, InitializeFn)(this, inData);

    let skill_detail = GameObject::GetComponentInChildren(
        Component::get_gameObject(this), 
        DialogCharacterSimpleSkillDetail::type_object(), 
        true
    );
    if skill_detail.is_null() {
        info!("skill_detail null");
        return;
    }

    let skill_item = DialogCharacterSimpleSkillDetail::get__partsSingleModeSkillListItem(skill_detail);
    if skill_item.is_null() {
        info!("skill_item null");
        return;
    }

    let desc_text = PartsSingleModeSkillListItem::get__descText(skill_item);
    let bg_btn = PartsSingleModeSkillListItem::get__bgButton(skill_item);
    
    if !desc_text.is_null() && !bg_btn.is_null() {
        Text::set_horizontalOverflow(desc_text, 0); 
        Text::set_verticalOverflow(desc_text, 1);
        let needed_height = Text::get_preferredHeight(desc_text);
        let final_height = needed_height + 150.0;

        let bg_obj = Component::get_gameObject(bg_btn);
        let mut layout_element = GameObject::GetComponent(bg_obj, LayoutElement::type_object());
        if layout_element.is_null() {
            layout_element = GameObject::AddComponent(bg_obj, LayoutElement::type_object());
        }
        LayoutElement::set_minHeight(layout_element, final_height);

        let contents_root = DialogCommonBase::get_ContentsRoot(this);
        if !contents_root.is_null() {
            LayoutRebuilder::ForceRebuildLayoutImmediate(contents_root);
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCommon);

    let Initialize_addr = get_method_addr(DialogCommon, c"Initialize", 1);
    new_hook!(Initialize_addr, Initialize);

    Data::init(DialogCommon)
}