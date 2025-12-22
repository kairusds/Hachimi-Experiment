use crate::il2cpp::{symbols::{get_method_addr, get_field_from_name, get_field_ptr}, types::*};

#[cfg(target_os = "android")]
use crate::core::Hachimi;

static mut GET_ORIGINALSCREENWIDTH_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_OriginalScreenWidth, GET_ORIGINALSCREENWIDTH_ADDR, i32,);

static mut GET_ORIGINALSCREENHEIGHT_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_OriginalScreenHeight, GET_ORIGINALSCREENHEIGHT_ADDR, i32,);

#[cfg(target_os = "android")]
type SetResolutionFn = extern "C" fn(w: i32, h: i32, fullscreen: bool, forceUpdate: bool, skipKeepAspect: bool);
#[cfg(target_os = "android")]
fn SetResolution(w: i32, h: i32, fullscreen: bool, forceUpdate: bool, skipKeepAspect: bool) {
    get_orig_fn!(SetResolution, SetResolutionFn)(w.max(h), w.min(h), fullscreen, forceUpdate, skipKeepAspect);
}

// type get_IsLandscapeMode = extern "C" fn() -> bool;
#[cfg(target_os = "android")]
fn get_IsLandscapeMode() -> bool {
    true
}

// type get_ScreenOrientation = extern "C" fn() -> ScreenOrientation;
#[cfg(target_os = "android")]
fn get_ScreenOrientation() -> ScreenOrientation {
    ScreenOrientation_Landscape
}

// type get_IsVertical = extern "C" fn() -> bool;
#[cfg(target_os = "android")]
extern "C" fn get_IsVertical() -> bool {
    false
}

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
    unsafe { std::mem::zeroed() }
    /*
    let enumerator = get_orig_fn!(ChangeScreenOrientationPortraitAsync, ChangeScreenOrientationPortraitAsyncFn)();
    if Hachimi::instance().config.load().ui_scale == 1.0 { return enumerator; }

    if let Err(e) = enumerator.hook_move_next(ChangeScreenOrientationPortraitAsync_MoveNext) {
        error!("Failed to hook enumerator: {}", e);
    }

    enumerator
    */
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
        let SetResolution_addr = get_method_addr(Screen, c"SetResolution", 5);
        let get_IsLandscapeMode_addr = get_method_addr(Screen, c"get_IsLandscapeMode", 0);
        let get_IsVertical_addr = get_method_addr(Screen, c"get_IsVertical", 0);

        new_hook!(ChangeScreenOrientationLandscapeAsync_addr, ChangeScreenOrientationLandscapeAsync);
        new_hook!(ChangeScreenOrientationPortraitAsync_addr, ChangeScreenOrientationPortraitAsync);
        new_hook!(SetResolution_addr, SetResolution);
        new_hook!(get_IsLandscapeMode_addr, get_IsLandscapeMode);
        new_hook!(get_IsVertical_addr, get_IsVertical);
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
        GET_ORIGINALSCREENWIDTH_ADDR = get_method_addr(Screen, c"get_OriginalScreenWidth", 0);
        GET_ORIGINALSCREENHEIGHT_ADDR = get_method_addr(Screen, c"get_OriginalScreenHeight", 0);
    }
}