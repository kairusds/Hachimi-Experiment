use serde::{Deserialize, Serialize};
use crate::{il2cpp::{symbols::get_method_overload_addr, types::*}};

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
    keyboardType: i32,
    autocorrection: bool,
    multiline: bool,
    secure: bool
);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, TouchScreenKeyboard);

    unsafe {
        TOUCHSCREENKEYBOARD_OPEN_ADDR = il2cpp_resolve_icall(
            c"UnityEngine.TouchScreenKeyboard::Open(System.String,UnityEngine.TouchScreenKeyboardType,System.Boolean,System.Boolean,System.Boolean)".as_ptr()
        );
    }
}
