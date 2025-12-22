use crate::il2cpp::{symbols::get_method_addr, types::*};

#[cfg(target_os = "android")]
use crate::core::Hachimi;

static mut GET_SCREENORIENTATION_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_ScreenOrientation, GET_SCREENORIENTATION_ADDR, ScreenOrientation,);

static mut SET_RESOLUTION_ADDR: usize = 0;
impl_addr_wrapper_fn!(SetResolution, SET_RESOLUTION_ADDR, (), w: i32, h: i32, fullscreen: bool, forceUpdate: bool, skipKeepAspect: bool);

static mut INITIALIZE_CHANGE_ORIENTATION_ADDR: usize = 0;
impl_addr_wrapper_fn!(InitializeChangeOrientationForUIManager, INITIALIZE_CHANGE_ORIENTATION_ADDR, (), isPortrait: bool, bgCameraSettings: *mut u8);

static mut GET_ORIGINALSCREENWIDTH_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_OriginalScreenWidth, GET_ORIGINALSCREENWIDTH_ADDR, i32);

static mut GET_ORIGINALSCREENHEIGHT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_OriginalScreenHeight, GET_ORIGINALSCREENHEIGHT_ADDR, i32);

static mut BG_CAMERA_SETTINGS_ADDR: usize = 0;

#[cfg(target_os = "android")]
pub fn force_landscape() {
    let w = get_OriginalScreenWidth();
    let h = get_OriginalScreenHeight();
    SetResolution(w.max(h), w.min(h), true, true, true);
    InitializeChangeOrientationForUIManager(false, BG_CAMERA_SETTINGS_ADDR);
    super::UIManager::apply_ui_scale();
}

/*
#[cfg(target_os = "android")]
pub fn start_ChangeScreenOrientationLandscapeAsync() {
    let enumerator = ChangeScreenOrientationLandscapeAsync();
    let ui_manager = super::UIManager::instance();

    if !ui_manager.is_null() && !enumerator.this.is_null() {
        crate::il2cpp::hook::UnityEngine_CoreModule::MonoBehaviour::StartCoroutine(ui_manager, enumerator.this as *mut Il2CppObject);
    }
}*/

#[cfg(target_os = "android")]
extern "C" fn ChangeScreenOrientationLandscapeAsync_MoveNext(enumerator: *mut Il2CppObject) -> bool {
    use crate::il2cpp::symbols::MoveNextFn;
    let moved = get_orig_fn!(ChangeScreenOrientationLandscapeAsync_MoveNext, MoveNextFn)(enumerator);
    if !moved {
        super::UIManager::apply_ui_scale();
    }
    moved
}

#[cfg(target_os = "android")]
extern "C" fn ChangeScreenOrientationPortraitAsync_MoveNext(enumerator: *mut Il2CppObject) -> bool {
    use crate::il2cpp::symbols::MoveNextFn;
    let moved = get_orig_fn!(ChangeScreenOrientationPortraitAsync_MoveNext, MoveNextFn)(enumerator);
    if !moved {
        super::UIManager::apply_ui_scale();
    }
    moved
}

#[cfg(target_os = "android")]
type ChangeScreenOrientationLandscapeAsyncFn = extern "C" fn() -> crate::il2cpp::symbols::IEnumerator;
#[cfg(target_os = "android")]
extern "C" fn ChangeScreenOrientationLandscapeAsync() -> crate::il2cpp::symbols::IEnumerator {
    let enumerator = get_orig_fn!(ChangeScreenOrientationLandscapeAsync, ChangeScreenOrientationLandscapeAsyncFn)();
    if Hachimi::instance().config.load().ui_scale == 1.0 { return enumerator; }

    if let Err(e) = enumerator.hook_move_next(ChangeScreenOrientationLandscapeAsync_MoveNext) {
        error!("Failed to hook enumerator: {}", e);
    }

    enumerator
}

#[cfg(target_os = "android")]
type ChangeScreenOrientationPortraitAsyncFn = extern "C" fn() -> crate::il2cpp::symbols::IEnumerator;
#[cfg(target_os = "android")]
extern "C" fn ChangeScreenOrientationPortraitAsync() -> crate::il2cpp::symbols::IEnumerator {
    let enumerator = get_orig_fn!(ChangeScreenOrientationPortraitAsync, ChangeScreenOrientationPortraitAsyncFn)();
    if Hachimi::instance().config.load().ui_scale == 1.0 { return enumerator; }

    if let Err(e) = enumerator.hook_move_next(ChangeScreenOrientationPortraitAsync_MoveNext) {
        error!("Failed to hook enumerator: {}", e);
    }

    enumerator
}

#[cfg(target_os = "windows")]
type GetWidthFn = extern "C" fn() -> i32;
#[cfg(target_os = "windows")]
extern "C" fn get_Width() -> i32 {
    if let Some((width, _)) = crate::windows::utils::get_scaling_res() {
        return width;
    }

    get_orig_fn!(get_Width, GetWidthFn)()
}

#[cfg(target_os = "windows")]
pub fn get_Width_orig() -> i32 {
    get_orig_fn!(get_Width, GetWidthFn)()
}

#[cfg(target_os = "windows")]
type GetHeightFn = extern "C" fn() -> i32;
#[cfg(target_os = "windows")]
extern "C" fn get_Height() -> i32 {
    if let Some((_, height)) = crate::windows::utils::get_scaling_res() {
        return height;
    }

    get_orig_fn!(get_Height, GetHeightFn)()
}

#[cfg(target_os = "windows")]
pub fn get_Height_orig() -> i32 {
    get_orig_fn!(get_Height, GetHeightFn)()
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, Screen);

    // let get_OriginalScreenWidth_addr = get_method_addr(Screen, c"get_OriginalScreenWidth", 0);
    // let get_OriginalScreenHeight_addr = get_method_addr(Screen, c"get_OriginalScreenHeight", 0);

    // new_hook!(get_OriginalScreenWidth_addr, get_OriginalScreenWidth);
    // new_hook!(get_OriginalScreenHeight_addr, get_OriginalScreenHeight);

    #[cfg(target_os = "android")]
    {
        let ChangeScreenOrientationLandscapeAsync_addr = get_method_addr(Screen, c"ChangeScreenOrientationLandscapeAsync", 0);
        let ChangeScreenOrientationPortraitAsync_addr = get_method_addr(Screen, c"ChangeScreenOrientationPortraitAsync", 0);

        new_hook!(ChangeScreenOrientationLandscapeAsync_addr, ChangeScreenOrientationLandscapeAsync);
        new_hook!(ChangeScreenOrientationPortraitAsync_addr, ChangeScreenOrientationPortraitAsync);
    }

    #[cfg(target_os = "windows")]
    {
        let get_Width_addr = get_method_addr(Screen, c"get_Width", 0);
        let get_Height_addr = get_method_addr(Screen, c"get_Height", 0);

        new_hook!(get_Width_addr, get_Width);
        new_hook!(get_Height_addr, get_Height);
    }

    #[cfg(target_os = "android")]
    unsafe {
        GET_SCREENORIENTATION_ADDR = get_method_addr(Screen, c"get_ScreenOrientation", 0);
        GET_ORIGINALSCREENWIDTH_ADDR = get_method_addr(Screen, c"get_OriginalScreenWidth", 0);
        GET_ORIGINALSCREENHEIGHT_ADDR = get_method_addr(Screen, c"get_OriginalScreenHeight", 0);
        SET_RESOLUTION_ADDR = get_method_addr(Screen, c"SetResolution", 5);
        INITIALIZE_CHANGE_ORIENTATION_ADDR = get_method_addr(Screen, c"InitializeChangeOrientationForUIManager", 2);
        BG_CAMERA_SETTINGS_ADDR = get_field_value(Screen, c"_bgCameraSettings");
    }
}