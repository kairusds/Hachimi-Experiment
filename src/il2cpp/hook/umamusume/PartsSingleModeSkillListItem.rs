use crate::{
    core::{gui::SimpleMessageWindow, Gui, Hachimi, game::Region, utils::mul_int},
    il2cpp::{ext::Il2CppStringExt, hook::{UnityEngine_CoreModule::UnityAction, UnityEngine_UI::Text}, sql::{self, TextDataQuery}, symbols::{create_delegate, get_field_from_name, get_field_object_value, get_method_addr, GCHandle}, types::*}
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use super::ButtonCommon;
use fnv::FnvHashMap;

static CALLBACK_HANDLES: Lazy<Mutex<Vec<GCHandle>>> = Lazy::new(|| Mutex::default());
// static mut CURRENT_SKILL: Lazy<Mutex<Option<(String, String)>>> = Lazy::new(|| Mutex::new(None));
static SKILL_DATA_MAP: Lazy<Mutex<FnvHashMap<usize, i32>>> = Lazy::new(|| Mutex::default());

// SkillListItem
static mut NAMETEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__nameText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { NAMETEXT_FIELD })
}
static mut DESCTEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__descText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { DESCTEXT_FIELD })
}
static mut _BGBUTTON_FIELD: *mut FieldInfo = 0 as _;
fn get__bgButton(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _BGBUTTON_FIELD })
}

// SkillInfo
static mut get_IsDrawDesc_addr: usize = 0;
impl_addr_wrapper_fn!(get_IsDrawDesc, get_IsDrawDesc_addr, bool, this: *mut Il2CppObject);
static mut get_IsDrawNeedSkillPoint_addr: usize = 0;
impl_addr_wrapper_fn!(get_IsDrawNeedSkillPoint, get_IsDrawNeedSkillPoint_addr, bool, this: *mut Il2CppObject);
static mut get_Id_addr: usize = 0;
impl_addr_wrapper_fn!(get_Id, get_Id_addr, i32, this: *mut Il2CppObject);

fn UpdateItemCommon(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, orig_fn_cb: impl FnOnce()) {
    let skill_cfg = &Hachimi::instance().localized_data.load().config.skill_formatting;
    let mut txt_cfg = sql::SkillTextFormatting::default();

    let name = get__nameText(this);
    let desc = get__descText(this);

    // Name should always exist, but let's be sure.
    if !name.is_null() {
        let mut name_len = skill_cfg.name_length;
        let mut name_lines = 1;

        // Uma info, "short ver".
        if !get_IsDrawDesc(skill_info) {
            name_len = mul_int(name_len, skill_cfg.name_short_mult);
            name_lines = skill_cfg.name_short_lines;
        }
        // "Draw Skill Pt" is also true on the short ver, even though it doesn't show there.
        // So, apply only when desc shows.
        else if get_IsDrawNeedSkillPoint(skill_info) {
            name_len = mul_int(name_len, skill_cfg.name_sp_mult);
        }
        // todo: When lvl display!?
        // if get_IsDrawUniqSkillInfo(skill_info) || get_Level(skill_info) > 1 {
        //     name_len = mul_int(name_len, skill_cfg.name_lvl_mult);
        // }

        txt_cfg.name = Some(sql::TextFormatting {
            line_len: name_len,
            line_count: name_lines,
            font_size: Text::get_fontSize(name)
        });
    }

    if get_IsDrawDesc(skill_info) && !desc.is_null() {
        let mut desc_len = skill_cfg.desc_length;
        // todo: When conditions button!?
        // if get_IsDisplayUpgradeSkill(skill_info) {
        //     desc_len = mul_int(desc_len, skill_cfg.desc_btn_mult);
        // }

        txt_cfg.desc = Some(sql::TextFormatting {
            line_len: desc_len,
            line_count: 4,
            font_size: Text::get_fontSize(desc)
        });
    }

    TextDataQuery::with_skill_query(&txt_cfg, orig_fn_cb);

    if txt_cfg.is_localized {
        if !name.is_null() {
            Text::set_horizontalOverflow(name, 1);
            if txt_cfg.name.map(|opts| opts.line_count).unwrap_or(1) > 1 {
                Text::set_verticalOverflow(name, 1);
            }
        }
        if !desc.is_null() {
            Text::set_horizontalOverflow(desc, 1);
        }
    }
}

type UpdateItemJpFn = extern "C" fn(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, resource_hash: i32);
extern "C" fn UpdateItemJp(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool, resource_hash: i32) {
    UpdateItemCommon(this, skill_info, || {
        get_orig_fn!(UpdateItemJp, UpdateItemJpFn)(this, skill_info, is_plate_effect_enable, resource_hash);
    });
}

type UpdateItemOtherFn = extern "C" fn(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool);
extern "C" fn UpdateItemOther(this: *mut Il2CppObject, skill_info: *mut Il2CppObject, is_plate_effect_enable: bool) {
    UpdateItemCommon(this, skill_info, || {
        get_orig_fn!(UpdateItemOther, UpdateItemOtherFn)(this, skill_info, is_plate_effect_enable);
    });
}

// private Void SetupOnClickSkillButton(Info skillInfo) { }
// type SetupOnClickSkillButtonFn = extern "C" fn(this: *mut Il2CppObject, info: *mut Il2CppObject);
extern "C" fn SetupOnClickSkillButton(this: *mut Il2CppObject, info: *mut Il2CppObject) {
    // ACTION_DATA_MAP.lock().unwrap().insert(callback_ptr as usize, (name, desc));

    // let handle = GCHandle::new(callback_ptr as *mut Il2CppObject, false);
    // CALLBACK_HANDLES.lock().unwrap().push(GCHandle::new(callback_ptr as _, false));

    let button = get__bgButton(this);
    let delegate = create_delegate(unsafe { UnityAction::UNITYACTION_CLASS }, 0, OnSkillClicked);
    SKILL_DATA_MAP.lock().unwrap().insert(delegate as usize, skill_id);
    CALLBACK_HANDLES.lock().unwrap().push(GCHandle::new(delegate, false));
    ButtonCommon::SetOnClick(button, delegate);
    // get_orig_fn!(SetupOnClickSkillButton, SetupOnClickSkillButtonFn)(this, skill_info);
}

extern "C" fn OnSkillClicked(delegate_obj: *mut Il2CppObject) {
    let skill_id = {
        let map = SKILL_DATA_MAP.lock().unwrap();
        *map.get(&(delegate_obj as usize)).unwrap_or(&0)
    };

    if skill_id == 0 { return; }

    if let Some(mutex) = Gui::instance() {
        let to_s = |opt_ptr: Option<*mut Il2CppString>| unsafe {
            opt_ptr.and_then(|p| p.as_ref()).map(|s| s.as_utf16str().to_string())
        };

        let skill_name = to_s(TextDataQuery::get_skill_name(skill_id)).unwrap_or_else(|| "Skill".to_string());
        let skill_desc = to_s(TextDataQuery::get_skill_desc(skill_id)).unwrap_or_else(|| "No description".to_string());

        mutex.lock().unwrap().show_window(Box::new(SimpleMessageWindow::new(
            &skill_name,
            &skill_desc
        )));
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, PartsSingleModeSkillListItem);
    find_nested_class_or_return!(PartsSingleModeSkillListItem, Info);

    if Hachimi::instance().game.region == Region::Japan {
        let UpdateItem_addr = get_method_addr(PartsSingleModeSkillListItem, c"UpdateItem", 3);
        new_hook!(UpdateItem_addr, UpdateItemJp);
    }
    else {
        let UpdateItem_addr = get_method_addr(PartsSingleModeSkillListItem, c"UpdateItem", 2);
        new_hook!(UpdateItem_addr, UpdateItemOther);
    }

    let SetupOnClickSkillButton_addr = get_method_addr(PartsSingleModeSkillListItem, c"SetupOnClickSkillButton", 1);
    new_hook!(SetupOnClickSkillButton_addr, SetupOnClickSkillButton);

    unsafe {
        NAMETEXT_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_nameText");
        DESCTEXT_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_descText");
        _BGBUTTON_FIELD = get_field_from_name(PartsSingleModeSkillListItem, c"_bgButton");

        // SkillInfo
        get_IsDrawDesc_addr = get_method_addr(Info, c"get_IsDrawDesc", 0);
        get_IsDrawNeedSkillPoint_addr = get_method_addr(Info, c"get_IsDrawNeedSkillPoint", 0);
        get_Id_addr = get_method_addr(Info, c"get_Id", 0);
    }
}
