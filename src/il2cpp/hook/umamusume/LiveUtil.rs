use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*}
};

unsafe fn force_array_ids(array_ptr: *mut Il2CppArray, ids: &[i32]) {
    if array_ptr.is_null() {
        return;
    }

    let len = (*array_ptr).max_length as usize;
    let data_ptr = array_ptr.add(1) as *mut i32;
    for i in 0..len.min(ids.len()) {
        *data_ptr.add(i) = ids[i];
    }
}

type GetSingCharaIdListFn = extern "C" fn(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetSingCharaIdList(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject {
    unsafe {
        if !vocalCharaIdArray.is_null() {
            /* let array = &*vocalCharaIdArray;
            let len = array.max_length as usize;
            let data_ptr = vocalCharaIdArray.add(1) as *mut i32; */

            /* force index 0 to a specific ID
            if len > 0 {
                *data_ptr.add(0) = 1001;
            }
            */

            if songId == 1151 { // legend changer
                // fill indices 0 to 2 with specific IDs
                // 1097 still, 1075 tact, 9002 akikawa?
                let chara_ids = [1097, 1075, 9002];
                force_array_ids(allCharaIdArray, &chara_ids);
                // 1129 amoai, 1116 donna, 1135 stego
                let chara_vo_ids = [1129, 1116, 1135];
                force_array_ids(vocalCharaIdArray, &chara_vo_ids);
                /* let chara_ids = [1129, 1116, 1135];
                for i in 0..len.min(3) {
                    *data_ptr.add(i) = chara_ids[i];
                } */
            }
        }
    }

    get_orig_fn!(GetSingCharaIdList, GetSingCharaIdListFn)(songId, songPartNumber, allCharaIdArray, vocalCharaIdArray, shuffledCharaDataList)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", LiveUtil);

    let GetSingCharaIdList_addr = get_method_addr(LiveUtil, c"GetSingCharaIdList", 5);
    new_hook!(GetSingCharaIdList_addr, GetSingCharaIdList);
}

