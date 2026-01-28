use crate::il2cpp::{ext::Il2CppStringExt, symbols::get_method_addr, types::*};
use std::sync::atomic::Ordering;
use super::GameSystem::GAME_INITIALIZED;

static mut LAST_ATLAS_PTR: *mut Il2CppString = std::ptr::null_mut();
static mut LAST_SPRITE_PTR: *mut Il2CppString = std::ptr::null_mut();

type GetSpriteFromNameSubFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteFromNameSub(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasName != unsafe { LAST_ATLAS_PTR } || spriteName != unsafe { LAST_SPRITE_PTR } {
            let atlas = unsafe { atlasName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            let sprite = unsafe { spriteName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            info!("GetSpriteFromNameSubFn (string, string): {}, {}", atlas, sprite);
            unsafe {
                LAST_ATLAS_PTR = atlasName;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }
    get_orig_fn!(GetSpriteFromNameSub, GetSpriteFromNameSubFn)(atlasName, spriteName)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, AtlasUtil);

    let GetSpriteFromNameSub_addr = get_method_addr(AtlasUtil, c"GetSpriteFromNameSub", 2);
    new_hook!(GetSpriteFromNameSub_addr, GetSpriteFromNameSub);
}
