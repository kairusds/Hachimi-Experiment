use crate::il2cpp::{symbols::get_method_addr, types::*};

pub extern "C" fn get_msaa(this: *mut Il2CppObject) -> bool {
    true
}

pub fn init(image: *const Il2CppImage) {
    get_class_or_return!(image, "", RenderingFeatures);

    unsafe {
        let get_msaa_addr = get_method_addr(RenderingFeatures, c"get_msaa", 0);
        new_hook!(get_msaa_addr, get_msaa);
    }
}
