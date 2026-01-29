use crate::il2cpp::types::*;

#[repr(i32)]
#[allow(dead_code)]
pub enum KeyboardType {
    Default,
    ASCIICapable,
    NumbersAndPunctuation,
    URL,
    NumberPad,
    PhonePad,
    NamePhonePad,
    EmailAddress,
    NintendoNetworkAccount,
    Social,
    Search,
    DecimalPad,
    OneTimeCode
}

pub fn init(_UnityEngine_CoreModule: *const Il2CppImage) {
}