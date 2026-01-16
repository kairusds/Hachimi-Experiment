use serde::{Deserialize, Serialize};
use crate::{il2cpp::{symbols::get_method_addr, types::*}};

static mut TOUCHSCREENKEYBOARD_OPEN_ADDR: usize = 0;

// UnityEngine 
#[derive(Default, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[repr(i32)]
pub enum TouchScreenKeyboardType {
    #[default] Default = 0,
    ASCIICapable = 1,
    NumbersAndPunctuation = 2,
    URL = 3,
    NumberPad = 4,
    PhonePad = 5,
    NamePhonePad = 6,
    EmailAddress = 7,
    NintendoNetworkAccount = 8,
    Social = 9,
    Search = 10,
    DecimalPad = 11,
    OneTimeCode = 12
}

impl_addr_wrapper_fn!(
    Open, 
    TOUCHSCREENKEYBOARD_OPEN_ADDR, 
    *mut Il2CppObject, 
    text: *mut Il2CppString,
    keyboardType: TouchScreenKeyboardType,
    autocorrection: bool,
    multiline: bool,
    secure: bool,
    alert: bool,
    textPlaceholder: *mut Il2CppString,
    characterLimit: i32
);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, TouchScreenKeyboard);

    unsafe {
        TOUCHSCREENKEYBOARD_OPEN_ADDR = get_method_addr(TouchScreenKeyboard, c"Open", 8);
    }
}
