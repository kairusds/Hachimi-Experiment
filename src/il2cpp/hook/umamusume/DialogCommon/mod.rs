use crate::{
    il2cpp::{
        ext::Il2CppStringExt,
        hook::{
            umamusume::{DialogCharacterSimpleSkillDetail, DialogCommonBase, DialogObject, ImageCommon, PartsSingleModeSkillListItem, TextCommon},
            UnityEngine_CoreModule::{Component, Object, GameObject, RectOffset, RectTransform, Transform},
            UnityEngine_UI::{ContentSizeFitter, HorizontalOrVerticalLayoutGroup, Image, LayoutElement, LayoutGroup, LayoutRebuilder, Text, VerticalLayoutGroup}
        },
        symbols::{Array, get_field_from_name, get_field_object_value, get_method_addr},
        types::*
    }
};

pub mod Data;

static mut _DIALOGOBJARRAY_FIELD: *mut FieldInfo = 0 as _;
pub fn get__dialogObjArray(this: *mut Il2CppObject) -> *mut Il2CppArray {
    get_field_object_value(this, unsafe { _DIALOGOBJARRAY_FIELD })
}

type InitializeFn = extern "C" fn(this: *mut Il2CppObject, inData: *mut Il2CppObject);
extern "C" fn Initialize(this: *mut Il2CppObject, inData: *mut Il2CppObject) {
    get_orig_fn!(Initialize, InitializeFn)(this, inData);

    let array_ptr = get__dialogObjArray(this);
    info!("array_ptr {:p}", array_ptr);

    if !array_ptr.is_null() {
        unsafe {
            let length = (*array_ptr).max_length;
            info!("array_ptr max length {}", length);

            let element_size = std::mem::size_of::<*mut Il2CppObject>(); 
            let data_ptr = (array_ptr as *mut u8).add(0x20);

            for i in 0..length {
                let slot_ptr = data_ptr.add(i as usize * element_size) as *mut *mut Il2CppObject;
                let dialog_obj = unsafe { *slot_ptr }; // DialogObject
                info!("dialog_obj {:p}", dialog_obj);

                let base_rect = DialogObject::get__baseRectTransform(dialog_obj);
                info!("base_rect {:p}", base_rect);

                let base_game_obj = Component::get_gameObject(base_rect);
                let transform = GameObject::get_transform(base_game_obj);
                let child_count = Transform::get_childCount(transform);
                info!("transform child_count {}", child_count);

                for j in 0..child_count {
                    let child = Transform::GetChild(transform, j);
                    let child_go = Component::get_gameObject(child);
                    let obj_name = Object::get_name(child_go);
                    let name_str = unsafe { (*obj_name).as_utf16str() }.to_string();
                    info!("name_str {}", name_str);

                    // Found the skill container GEEEGEEEEEEEE
                    if name_str == "Skill" {
                        info!("im inside skill");
                        let mut vlg = GameObject::GetComponent(child_go, VerticalLayoutGroup::type_object());
                        if vlg.is_null() {
                            vlg = GameObject::AddComponent(child_go, VerticalLayoutGroup::type_object());
                            info!("add missing vlg to skill");
                        }
                        HorizontalOrVerticalLayoutGroup::set_childControlHeight(vlg, false);
                        HorizontalOrVerticalLayoutGroup::set_childForceExpandHeight(vlg, false);
                        LayoutGroup::set_padding(vlg, RectOffset::new(20, 20, 20, 20));

                        let img_comp = GameObject::GetComponent(child_go, ImageCommon::type_object());
                        if !img_comp.is_null() {
                            Image::set_type(img_comp, 1); // Sliced
                        }
                        
                        let mut csf = GameObject::GetComponent(child_go, ContentSizeFitter::type_object());
                        if csf.is_null() {
                            info!("why csf null");
                            csf = GameObject::AddComponent(child_go, ContentSizeFitter::type_object());
                        }
                        ContentSizeFitter::set_verticalFit(csf, 2); // PreferredSize

                        LayoutRebuilder::ForceRebuildLayoutImmediate(base_rect);
                    }
                }
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCommon);
    Data::init(DialogCommon);

    let Initialize_addr = get_method_addr(DialogCommon, c"Initialize", 1);
    new_hook!(Initialize_addr, Initialize);

    unsafe {
        _DIALOGOBJARRAY_FIELD = get_field_from_name(DialogCommon, c"_dialogObjArray");
    }
}