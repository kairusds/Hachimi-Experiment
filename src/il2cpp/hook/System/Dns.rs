use crate::{core::Hachimi, il2cpp::{symbols::get_method_addr, types::*}};
use std::ptr::null_mut;
use super::Socket::InterNetworkV6;

// System.Net, IPAddress
// public AddressFamily get_AddressFamily() { }
static mut GET_ADDRESSFAMILY_ADDR: usize = 0;
impl_addr_wrapper_fn!(get_AddressFamily, GET_ADDRESSFAMILY_ADDR, i32, this: *mut Il2CppObject);

// public static IPAddress[] GetHostAddresses(String hostNameOrAddress) { }
type GetHostAddressesFn = extern "C" fn(this: *mut Il2CppObject, hostNameOrAddress: *mut Il2CppString) -> *mut Il2CppArray;
extern "C" fn GetHostAddresses(this: *mut Il2CppObject, hostNameOrAddress: *mut Il2CppString) -> *mut Il2CppArray{
    let result = get_orig_fn!(GetHostAddresses, GetHostAddressesFn)(this, hostNameOrAddress);

    if Hachimi::instance().config.load().ipv4_only && !result.is_null() {
        unsafe {
            let len = (*result).max_length as usize;
            let data_ptr = result.add(1) as *mut *mut Il2CppObject;

            for i in 0..len {
                let ip_obj = *data_ptr.add(i);
                if !ip_obj.is_null() {
                    let family = get_AddressFamily(ip_obj);
                    if family == InterNetworkV6 {
                        *data_ptr.add(i) = null_mut();
                    }
                }
            }
        }
    }

    return result;
}

pub fn init(system_img: *const Il2CppImage) {
    get_class_or_return!(system_img, "System.Net", Dns);
    get_class_or_return!(system_img, "System.Net", IPAddress);

    let GetHostAddresses_addr = get_method_addr(Dns, c"GetHostAddresses", 1);
    new_hook!(GetHostAddresses_addr, GetHostAddresses);

    unsafe {
        GET_ADDRESSFAMILY_ADDR = get_method_addr(IPAddress, c"get_AddressFamily", 0);
    }
}
