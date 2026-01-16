use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*}
};

unsafe fn force_array_ids(array_ptr: *mut Il2CppArray, ids: &[i32]) {
    if array_ptr.is_null() {
        return;
    }
    let array_len = (*array_ptr).max_length as usize;

    // Il2CppArray data starts right after the header (which is why it needs to add 1)
    let data_ptr = (array_ptr as *mut u8).add(std::mem::size_of::<Il2CppArray>()) as *mut i32;
    for i in 0..array_len.min(ids.len()) {
        let new_id = ids[i];
        if new_id == 0 {
            continue;
        }
        *data_ptr.add(i) = new_id;
    }
}

type GetSingCharaIdListFn = extern "C" fn(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetSingCharaIdList(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject {
    let chara_vo_ids = &Hachimi::instance().config.load().live_vocals_swap;

    unsafe {
        let vo_chara_len = (*vocalCharaIdArray).max_length as usize;
        info!("vo_chara_len: {)", vo_chara_len);
        let limit = vo_chara_len.min(chara_vo_ids.len());
        info!("limit: {)", limit);
        let truncated_ids = &chara_vo_ids[..limit];
        info!("truncated_ids len: {)", truncated_ids.len());

        force_array_ids(vocalCharaIdArray, truncated_ids);
    }

    get_orig_fn!(GetSingCharaIdList, GetSingCharaIdListFn)(songId, songPartNumber, allCharaIdArray, vocalCharaIdArray, shuffledCharaDataList)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", LiveUtil);

    let GetSingCharaIdList_addr = get_method_addr(LiveUtil, c"GetSingCharaIdList", 5);
    new_hook!(GetSingCharaIdList_addr, GetSingCharaIdList);
}

