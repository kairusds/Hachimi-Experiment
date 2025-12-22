use crate::il2cpp::{symbols::{get_method_overload_addr, IEnumerator}, types::*};

static mut START_COROUTINE_ADDR: usize = 0;
impl_addr_wrapper_fn!(StartCoroutine, START_COROUTINE_ADDR, *mut Il2CppObject, this: *mut Il2CppObject, routine: *mut Il2CppObject);

pub fn init(UnityEngine_CoreModule: *const Il2CppImage) {
    get_class_or_return!(UnityEngine_CoreModule, UnityEngine, MonoBehaviour);

    unsafe {
        START_COROUTINE_ADDR = get_method_overload_addr(MonoBehaviour, "StartCoroutine", &[Il2CppTypeEnum_IL2CPP_TYPE_CLASS]);
    }
}
