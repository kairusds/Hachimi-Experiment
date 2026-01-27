use crate::{
    il2cpp::{
        hook::{
            umamusume::{DialogCharacterSimpleSkillDetail, DialogCommonBase, DialogObject, ImageCommon, PartsSingleModeSkillListItem, TextCommon},
            UnityEngine_CoreModule::{Component, GameObject, RectOffset, RectTransform, Transform},
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

            let klass_ref: &mut *mut Il2CppClass =
                (&mut (*array_ptr).obj.__bindgen_anon_1.klass).as_mut();

            let element_size = (*(*klass_ref)).element_size as usize;

            let data_ptr = (array_ptr as *mut u8).add(kIl2CppSizeOfArray);

            for i in 0..length {
                let dialog_obj = data_ptr.add(i * element_size) as *mut Il2CppObject; // DialogObject
                info!("dialog_obj {:p}", dialog_obj);
                if dialog_obj.is_null(){ return; };

                let base_rect = DialogObject::get__baseRectTransform(dialog_obj);
                info!("base_rect {:p}", base_rect);
                if base_rect.is_null(){ return; };

                if !base_rect.is_null() {
                    let base_game_obj = Component::get_gameObject(base_rect);

                    let text_objects = GameObject::GetComponentsInChildren(dialog_obj, TextCommon::type_object(), true);                    
                    for text_obj in unsafe { text_objects.as_slice().iter() } {
                        // if !*text_obj.is_null() {
                        Text::set_horizontalOverflow(*text_obj, 0);
                        Text::set_verticalOverflow(*text_obj, 1);
                        // }
                    }

                    let img_objects = GameObject::GetComponentsInChildren(dialog_obj, ImageCommon::type_object(), true);
                    for img_obj in unsafe { img_objects.as_slice().iter() } {
                        // if !*img_obj.is_null() {
                        let img_go = Component::get_gameObject(*img_obj);
                        info!("img_go {:p}", img_go);
                        if img_go.is_null(){ return; };
                        let mut vlg = GameObject::GetComponent(img_go, VerticalLayoutGroup::type_object());
                        if vlg.is_null() {
                            info!("vlg null, maybe not the correct one");
                            continue;
                            // vlg = GameObject::AddComponent(img_go, VerticalLayoutGroup::type_object());
                        }
                        // sliced
                        Image::set_type(img_go, 1);

                        LayoutGroup::set_padding(vlg, RectOffset::new(30, 30, 30, 30));
                        HorizontalOrVerticalLayoutGroup::set_childControlHeight(vlg, true);
                        HorizontalOrVerticalLayoutGroup::set_childForceExpandHeight(vlg, false);

                        let mut csf = GameObject::GetComponent(img_go, ContentSizeFitter::type_object());
                        info!("csf before {:p}", csf);

                        if csf.is_null() {
                            csf = GameObject::AddComponent(img_go, ContentSizeFitter::type_object());
                            info!("csf empty making new one");
                        }
                        ContentSizeFitter::set_verticalFit(csf, 2); // PreferredSize
                        // }
                    }

                    LayoutRebuilder::ForceRebuildLayoutImmediate(base_rect);
                }

                let root_rect = DialogObject::get__rootRectTransform(dialog_obj);
                info!("root_rect {:p}", root_rect);
                if !root_rect.is_null() {
                    LayoutRebuilder::ForceRebuildLayoutImmediate(root_rect);
                }
            }
        }
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, DialogCommon);

    let Initialize_addr = get_method_addr(DialogCommon, c"Initialize", 1);
    new_hook!(Initialize_addr, Initialize);

    unsafe {
        _DIALOGOBJARRAY_FIELD = get_field_from_name(DialogCommon, c"_dialogObjArray");
    }

    Data::init(DialogCommon)
}