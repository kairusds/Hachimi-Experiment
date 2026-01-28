use crate::il2cpp::{ext::Il2CppStringExt, symbols::{get_method_addr, get_method_overload_addr}, types::*};

type GetSpriteByNameFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    info!("GetSpriteByName (string, string): {}, {}", unsafe { (*atlasName).as_utf16str() }.to_string(), unsafe { (*spriteName).as_utf16str() }.to_string());
    get_orig_fn!(GetSpriteByName, GetSpriteByNameFn)(atlasName, spriteName)
}

type GetSpriteByName1Fn = extern "C" fn(atlasType: i32, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName1(atlasType: i32, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    info!("GetSpriteByName (i32, string): {}, {}", atlasType, unsafe { (*spriteName).as_utf16str() }.to_string());
    get_orig_fn!(GetSpriteByName1, GetSpriteByName1Fn)(atlasType, spriteName)
}

type GetSpriteFromNameSubFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteFromNameSub(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    info!("GetSpriteByName (string, string): {}, {}", unsafe { (*atlasName).as_utf16str() }.to_string(), unsafe { (*spriteName).as_utf16str() }.to_string());
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
