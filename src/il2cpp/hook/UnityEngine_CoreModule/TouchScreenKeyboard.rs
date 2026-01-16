use serde::{Deserialize, Serialize};
use crate::{il2cpp::{symbols::get_method_overload_addr, types::*}};
use supper::TouchScreenKeyboardType;

static mut TOUCHSCREENKEYBOARD_OPEN_ADDR: usize = 0;

impl_addr_wrapper_fn!(
    Open, 
    TOUCHSCREENKEYBOARD_OPEN_ADDR, 
    *mut Il2CppObject, 
    text: *mut Il2CppString,
    keyboardType: TouchScreenKeyboardType::KeyboardType,
    autocorrection: bool,
    multiline: bool,
    secure: bool
);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, TouchScreenKeyboard);

    unsafe {
        TOUCHSCREENKEYBOARD_OPEN_ADDR = get_method_overload_addr(
            TouchScreenKeyboard, 
            "Open", 
            &[
                Il2CppTypeEnum_IL2CPP_TYPE_STRING,   // String text
                Il2CppTypeEnum_IL2CPP_TYPE_VALUETYPE, // TouchScreenKeyboardType (Enum)
                Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN,  // Boolean autocorrection
                Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN,  // Boolean multiline
                Il2CppTypeEnum_IL2CPP_TYPE_BOOLEAN,  // Boolean secure
            ]
        );
    }
}
