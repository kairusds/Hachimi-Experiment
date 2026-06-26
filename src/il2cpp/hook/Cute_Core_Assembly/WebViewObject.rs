use crate::il2cpp::{
	ext::Il2CppStringExt,
	symbols::get_method_addr, types::*
};

type LoadURLFn = extern "C" fn(this: *mut Il2CppObject, url: *mut Il2CppString);
pub extern "C" fn LoadURL(this: *mut Il2CppObject, url: *mut Il2CppString) {
	info!("WebViewObject LoadURL: {}", unsafe { (*url).as_utf16str().to_string() });
    get_orig_fn!(LoadURL, LoadURLFn)(this, url);
}

pub fn init(Cute_Core_Assembly: *const Il2CppImage) {
    get_class_or_return!(Cute_Core_Assembly, "", WebViewObject);

    let LoadURL_addr = get_method_addr(WebViewObject, c"LoadURL", 1);
    new_hook!(LoadURL_addr, LoadURL);
}
