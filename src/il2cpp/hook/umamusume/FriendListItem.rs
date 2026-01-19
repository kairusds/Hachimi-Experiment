use crate::{core::Hachimi, il2cpp::{ext::StringExt, hook::UnityEngine_UI::Text, symbols::{get_field_from_name, get_field_object_value, get_method_addr}, types::*}};

// FriendListItem
static mut _USERNAMETEXT_FIELD: *mut FieldInfo = 0 as _;
fn get__userNameText(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _USERNAMETEXT_FIELD })
}

type ApplyCharacterButtonFn = extern "C" fn(this: *mut Il2CppObject, info: *mut Il2CppObject);
extern "C" fn ApplyCharacterButton(this: *mut Il2CppObject, info: *mut Il2CppObject) {
    get_orig_fn!(ApplyCharacterButton, ApplyCharacterButtonFn)(this, info);
    if Hachimi::instance().config.load().hide_usernames {
        let username_text = get__userNameText(this);
        Text::set_text(username_text, "".to_string().to_il2cpp_string());
    }
}

// FriendListItemAddFactor
static mut _USERNAMETEXT1_FIELD: *mut FieldInfo = 0 as _;
fn get__userNameText1(this: *mut Il2CppObject) -> *mut Il2CppObject {
    get_field_object_value(this, unsafe { _USERNAMETEXT1_FIELD })
}

type ApplyCharacterButton1Fn = extern "C" fn(this: *mut Il2CppObject, friendListItemInfo: *mut Il2CppObject);
extern "C" fn ApplyCharacterButton1(this: *mut Il2CppObject, friendListItemInfo: *mut Il2CppObject) {
    get_orig_fn!(ApplyCharacterButton, ApplyCharacterButtonFn)(this, friendListItemInfo);
    if Hachimi::instance().config.load().hide_usernames {
        let username_text = get__userNameText1(this);
        Text::set_text(username_text, "".to_string().to_il2cpp_string());
    }
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, FriendListItem);
    get_class_or_return!(umamusume, Gallop, FriendListItemAddFactor);

    let ApplyCharacterButton_addr = get_method_addr(FriendListItem, c"ApplyCharacterButton", 1);
    new_hook!(ApplyCharacterButton_addr, ApplyCharacterButton);

    let ApplyCharacterButton1_addr = get_method_addr(FriendListItemAddFactor, c"ApplyCharacterButton", 1);
    new_hook!(ApplyCharacterButton1_addr, ApplyCharacterButton1);

    unsafe {
        _USERNAMETEXT_FIELD = get_field_from_name(FriendListItem, c"_userNameText");
        _USERNAMETEXT1_FIELD = get_field_from_name(FriendListItemAddFactor, c"_userNameText");
    }
}
