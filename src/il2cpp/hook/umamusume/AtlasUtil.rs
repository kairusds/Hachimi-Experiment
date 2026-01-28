use crate::il2cpp::{ext::Il2CppStringExt, symbols::{get_method_addr, get_method_overload_addr}, types::*};
use std::sync::atomic::Ordering;
use super::GameSystem::GAME_INITIALIZED;

static mut LAST_ATLAS_PTR: *mut Il2CppString = std::ptr::null_mut();
static mut LAST_SPRITE_PTR: *mut Il2CppString = std::ptr::null_mut();
static mut LAST_TYPE: i32 = -1;

type GetSpriteByNameFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasName != unsafe { LAST_ATLAS_PTR } && spriteName != unsafe { LAST_SPRITE_PTR } {
            let atlas = if !atlasName.is_null() { unsafe { (*atlasName).as_utf16str() }.to_string() } else { "null".into() };
            let sprite = if !spriteName.is_null() { unsafe { (*spriteName).as_utf16str() }.to_string() } else { "null".into() };

            info!("GetSpriteByName (string, string): {}, {}", atlas, sprite);
            unsafe {
                LAST_ATLAS_PTR = atlasName;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }
    get_orig_fn!(GetSpriteByName, GetSpriteByNameFn)(atlasName, spriteName)
}

type GetSpriteByName1Fn = extern "C" fn(atlasType: i32, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName1(atlasType: i32, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasType != unsafe { LAST_TYPE } && spriteName != unsafe { LAST_SPRITE_PTR } {
            let sprite = if !spriteName.is_null() { unsafe { (*spriteName).as_utf16str() }.to_string() } else { "null".into() };
            info!("GetSpriteByName (i32, string): {}, {}", atlasType, sprite);
            unsafe {
                LAST_TYPE = atlasType;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }
    get_orig_fn!(GetSpriteByName1, GetSpriteByName1Fn)(atlasType, spriteName)
}

type GetSpriteFromNameSubFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteFromNameSub(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasName != unsafe { LAST_ATLAS_PTR } && spriteName != unsafe { LAST_SPRITE_PTR } {
            let atlas = if !atlasName.is_null() { unsafe { (*atlasName).as_utf16str() }.to_string() } else { "null".into() };
            let sprite = if !spriteName.is_null() { unsafe { (*spriteName).as_utf16str() }.to_string() } else { "null".into() };
            info!("GetSpriteFromNameSubFn (string, string): {}, {}", atlas, sprite);
            unsafe {
                LAST_ATLAS_PTR = atlasName;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }
    get_orig_fn!(GetSpriteFromNameSub, GetSpriteFromNameSubFn)(atlasName, spriteName)
}

// public static Sprite GetSpriteByName(String atlasName, String spriteName) { }
// public static Sprite GetSpriteByName(TargetAtlasType atlasType, String spriteName) { }
// private static Sprite GetSpriteFromNameSub(String atlasName, String spriteName) { }
	
    
pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, AtlasUtil);

    let GetSpriteByName_addr = get_method_overload_addr(AtlasUtil, "GetSpriteByName",
        &[Il2CppTypeEnum_IL2CPP_TYPE_STRING, Il2CppTypeEnum_IL2CPP_TYPE_STRING]);
    new_hook!(GetSpriteByName_addr, GetSpriteByName);

    let GetSpriteByName1_addr = get_method_overload_addr(AtlasUtil, "GetSpriteByName",
        &[Il2CppTypeEnum_IL2CPP_TYPE_VALUETYPE, Il2CppTypeEnum_IL2CPP_TYPE_STRING]);
    new_hook!(GetSpriteByName1_addr, GetSpriteByName1);

    let GetSpriteFromNameSub_addr = get_method_addr(AtlasUtil, c"GetSpriteFromNameSub", 2);
    new_hook!(GetSpriteFromNameSub_addr, GetSpriteFromNameSub);
}
