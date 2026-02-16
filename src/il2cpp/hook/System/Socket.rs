use crate::{core::Hachimi, il2cpp::{symbols::get_method_addr, types::*}};

// System.Net.Sockets, AddressFamily
pub const InterNetwork: i32 = 2; // IPv4
pub const InterNetworkV6: i32 = 23; // IPv6

// public Void .ctor(AddressFamily addressFamily, SocketType socketType, ProtocolType protocolType) { }
type ctorFn = extern "C" fn(this: *mut Il2CppObject, addressFamily: i32, socketType: *mut Il2CppObject, protocolType: *mut Il2CppObject);
extern "C" fn ctor(this: *mut Il2CppObject, addressFamily: i32, socketType: *mut Il2CppObject, protocolType: *mut Il2CppObject) {
    if Hachimi::instance().config.load().ipv4_only {
        return get_orig_fn!(ctor, ctorFn)(this, InterNetwork, socketType, protocolType);
    }
    get_orig_fn!(ctor, ctorFn)(this, addressFamily, socketType, protocolType);
}

pub fn init(System: *const Il2CppImage) {
    get_class_or_return!(System, "System.Net.Sockets", Socket);
    
    let ctor_addr = get_method_addr(Socket, c".ctor", 3);
    new_hook!(ctor_addr, ctor);
}
