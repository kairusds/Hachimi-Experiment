use crate::il2cpp::{
	ext::StringExt,
	symbols::get_method_addr, types::*
};

type LoadURLFn = extern "C" fn(this: *mut Il2CppObject, url: *mut Il2CppString);
pub extern "C" fn LoadURL(this: *mut Il2CppObject, _url: *mut Il2CppString) {
	let override_url = "https://example.com";
    get_orig_fn!(LoadURL, LoadURLFn)(this, override_url.to_il2cpp_string());
}

pub fn init(Cute_Core_Assembly: *const Il2CppImage) {
    get_class_or_return!(Cute_Core_Assembly, "", WebViewObject);

    let LoadURL_addr = get_method_addr(WebViewObject, c"LoadURL", 1);
    new_hook!(LoadURL_addr, LoadURL);
}
