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

// Dll : umamusume.dll
// Namespace: Gallop
//public abstract class DialogInnerBase : MonoBehaviour
//	private Data _dialogData;
static mut _DIALOGDATA_FIELD: *mut FieldInfo = 0 as _;
pub fn get__dialogData(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _DIALOGDATA_FIELD })
}

// Dll : umamusume.dll
// Namespace: Gallop
// public abstract class DialogCommonBase : LockableBehaviour, IAdjustSafeArea
// {
// private RectTransform _contentsRoot;
static mut _CONTENTSROOT_FIELD: *mut FieldInfo = 0 as _;
pub fn get__contentsRoot(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _CONTENTSROOT_FIELD })
}

// public DialogCommonBase GetDialog() { }
static mut GETDIALOG_ADDR: usize = 0;
impl_addr_wrapper_fn!(GetDialog, GETDIALOG_ADDR, *mut Il2CppObject, this: *mut Il2CppObject);

// private Void Setup(SkillData skillData, Boolean isDrawNeedSkillPoint, ShowHintLvUpParam showHintLvUpParam, Int32 skillUpgradeCardId, Boolean isSingleMode, Boolean isDisplayUpgradeSkill) { }
// public static Void Open(SkillData skillData, Boolean isDrawNeedSkillPoint, SkillLimitedType skillLimitedType, ShowHintLvUpParam hintLvUpParam, Int32 skillUpgradeCardId, Boolean isSingleMode, Boolean isDisplayUpgradeSkill) { }
type OpenFn = extern "C" fn(
    this: *mut Il2CppObject, // DialogCharacterSimpleSkillDetail
    skillData: *mut Il2CppObject,
    isDrawNeedSkillPoint: bool,
    skillLimitedType: *mut Il2CppObject,
    showHintLvUpParam: *mut Il2CppObject,
    skillUpgradeCardId: i32,
    isSingleMode: bool,
    isDisplayUpgradeSkill: bool
);
extern "C" fn Open(
    this: *mut Il2CppObject, // DialogCharacterSimpleSkillDetail
    skillData: *mut Il2CppObject,
    isDrawNeedSkillPoint: bool,
    skillLimitedType: *mut Il2CppObject,
    showHintLvUpParam: *mut Il2CppObject,
    skillUpgradeCardId: i32,
    isSingleMode: bool,
    isDisplayUpgradeSkill: bool
) {
    get_orig_fn!(Open, OpenFn)(this, skillData, isDrawNeedSkillPoint, skillLimitedType, showHintLvUpParam, skillUpgradeCardId, isSingleMode, isDisplayUpgradeSkill);

    let skill_item = get__partsSingleModeSkillListItem(this);
    if skill_item.is_null() {
        info!("DialogCharacterSimpleSkillDetail.Open skill_item is null");
    }
    let dialog = GetDialog(this);
    if dialog.is_null() {
        info!("dialog is null somehow");
    }

    /*
    let desc_text = PartsSingleModeSkillListItem::get__descText(skill_item);
    info!("desc_text: {:p}", desc_text);
    let bg_btn = PartsSingleModeSkillListItem::get__bgButton(skill_item);
    info!("bg_btn: {:p}", bg_btn);
    let bg_obj = Component::get_gameObject(bg_btn);

    if !desc_text.is_null() && !bg_obj.is_null() {
        Text::set_horizontalOverflow(desc_text, 0);
        Text::set_verticalOverflow(desc_text, 1);
        let needed_height = Text::get_preferredHeight(desc_text);
        info!("needed_height {}", needed_height);

        let mut layout_element = GameObject::GetComponent(bg_obj, LayoutElement::type_object());
        if layout_element.is_null() {
            info!("LayoutElement for _bgButton empty, building new one...");
            layout_element = GameObject::AddComponent(bg_obj, LayoutElement::type_object());
        }

        let image = GameObject::GetComponent(bg_obj, ImageCommon_Class);
        Image::set_type(image, 1);

        let final_height = needed_height + 150.0;
        info!("final_height; {}", final_height);
        LayoutElement::set_minHeight(layout_element, final_height);
        LayoutElement::set_flexibleHeight(layout_element, 0.0);

        let this_obj = Component::get_gameObject(this);
        if this.is_null() {
            info!("this_obj is NULL");
            return;
        }

        let dialog_data = get__dialogData(this);
        info!("dialog_data {:p}", dialog_data);
        let contents_root = 

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
    }*/
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCharacterSimpleSkillDetail);
    get_class_or_return!(umamusume, Gallop, DialogInnerBase);
    get_class_or_return!(umamusume, Gallop, DialogCommonBase);

    let Open_addr = get_method_addr(DialogCharacterSimpleSkillDetail, c"Open", 7);
    new_hook!(Open_addr, Open);

    unsafe {
        _PARTSSINGLEMODESKILLLISTITEM_FIELD = get_field_from_name(DialogCharacterSimpleSkillDetail, c"_partsSingleModeSkillListItem");
        _DIALOGDATA_FIELD = get_field_from_name(DialogInnerBase, c"_dialogData");
        _CONTENTSROOT_FIELD = get_field_from_name(DialogCommonBase, c"_contentsRoot");
        GETDIALOG_ADDR = get_method_addr(DialogInnerBase, c"GetDialog", 0);
        info!("_DIALOGDATA_FIELD: {:p}", _DIALOGDATA_FIELD);
    }
}
