use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*}
};

unsafe fn force_array_ids(array_ptr: *mut Il2CppArray, ids: &[i32]) {
    if array_ptr.is_null() {
        return;
    }

    let len = (*array_ptr).max_length as usize;
    // Il2CppArray data starts right after the header (which is why it needs to add 1)
    let data_ptr = array_ptr.add(1) as *mut i32;
    for i in 0..len.min(ids.len()) {
        if ids[i] == 0 {
            continue;
        }
        *data_ptr.add(i) = ids[i];
    }
}

type GetSingCharaIdListFn = extern "C" fn(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetSingCharaIdList(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject {
    let chara_vo_ids = Hachimi::instance().config.load().live_vocals_swap;
    unsafe {
        force_array_ids(vocalCharaIdArray, &chara_vo_ids);
    }

    get_orig_fn!(GetSingCharaIdList, GetSingCharaIdListFn)(songId, songPartNumber, allCharaIdArray, vocalCharaIdArray, shuffledCharaDataList)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", LiveUtil);

    let GetSingCharaIdList_addr = get_method_addr(LiveUtil, c"GetSingCharaIdList", 5);
    new_hook!(GetSingCharaIdList_addr, GetSingCharaIdList);
}

