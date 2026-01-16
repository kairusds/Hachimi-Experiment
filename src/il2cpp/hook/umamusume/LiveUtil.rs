use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*}
};

type GetSingCharaIdListFn = extern "C" fn(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetSingCharaIdList(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject {
    let result = get_orig_fn!(GetSingCharaIdList, GetSingCharaIdListFn)(
        songId, songPartNumber, allCharaIdArray, vocalCharaIdArray, shuffledCharaDataList
    );
    let chara_vo_ids = &Hachimi::instance().config.load().live_vocals_swap;

    unsafe {
        if !vocalCharaIdArray.is_null() {
            let len = (*vocalCharaIdArray).max_length as usize;
            info!("vocalCharaIdArray len {}", len);
            let data_ptr = vocalCharaIdArray.add(1) as *mut i32;
            
            for i in 0..len.min(chara_vo_ids.len()) {
                if let Some(data) = Hachimi::instance().chara_data.load().as_ref() {
                    info!("chara_vo_ids index {}: {}", i, data.get_name(chara_vo_ids[i]));
                }

                if chara_vo_ids[i] != 0 {
                    *data_ptr.add(i) = chara_vo_ids[i];
                }
            }
        }
    }

    result
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", LiveUtil);

    let GetSingCharaIdList_addr = get_method_addr(LiveUtil, c"GetSingCharaIdList", 5);
    new_hook!(GetSingCharaIdList_addr, GetSingCharaIdList);
}

