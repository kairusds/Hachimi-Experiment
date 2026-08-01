use crate::{
    core::Hachimi,
    il2cpp::{symbols::get_method_addr, types::*}
};

// public static void InitializeLoadSettings(int musicId, int sheetValiationId, MasterStoryLivePosition.ModelData[] modelDataArray) { }
type InitializeLoadSettingsFn = extern "C" fn(musicId: i32, sheetValiationId: i32, modelDataArray: *mut Il2CppObject);
extern "C" fn InitializeLoadSettings(musicId: i32, sheetValiationId: i32, modelDataArray: *mut Il2CppObject) {
    // get_orig_fn!(InitializeLoadSettings, InitializeLoadSettingsFn)(musicId, sheetValiationId, modelDataArray);
    info!("InitializeLoadSettings 1029");
    get_orig_fn!(InitializeLoadSettings, InitializeLoadSettingsFn)(1029, sheetValiationId, modelDataArray);
}

// public static void ChangeLiveSimple(int musicId, int sheetValiationId, MasterStoryLivePosition.ModelData[] modelDataArray, LiveViewController.ViewInfo viewInfo, Action onChangeViewCancel, Action preChangeViewAction) { }
type ChangeLiveSimpleFn = extern "C" fn(musicId: i32, sheetValiationId: i32, modelDataArray: *mut Il2CppObject, viewInfo: *mut Il2CppObject, onChangeViewCancel: *mut Il2CppObject, preChangeViewAction: *mut Il2CppObject);
extern "C" fn ChangeLiveSimple(musicId: i32, sheetValiationId: i32, modelDataArray: *mut Il2CppObject, viewInfo: *mut Il2CppObject, onChangeViewCancel: *mut Il2CppObject, preChangeViewAction: *mut Il2CppObject) {
    info!("ChangeLiveSimple 1029");
    get_orig_fn!(ChangeLiveSimple, ChangeLiveSimpleFn)(1029, sheetValiationId, modelDataArray, viewInfo, onChangeViewCancel, preChangeViewAction);
}

// public static void ChangeLive(int musicId, CharaDressIdSet[] idSetArray, LiveViewController.ViewInfo viewInfo, bool isSkipStory, Action onChangeViewCancel) { }
type ChangeLiveFn = extern "C" fn(musicId: i32, idSetArray: *mut Il2CppObject, viewInfo: *mut Il2CppObject, isSkipStory: bool, onChangeViewCancel: *mut Il2CppObject);
extern "C" fn ChangeLive(musicId: i32, idSetArray: *mut Il2CppObject, viewInfo: *mut Il2CppObject, isSkipStory: bool, onChangeViewCancel: *mut Il2CppObject) {
    info!("ChangeLive 1029");
    get_orig_fn!(ChangeLive, ChangeLiveFn)(1029, idSetArray, viewInfo, isSkipStory, onChangeViewCancel);
}

type GetSingCharaIdListFn = extern "C" fn(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject;
extern "C" fn GetSingCharaIdList(songId: i32, songPartNumber: i32, allCharaIdArray: *mut Il2CppArray, vocalCharaIdArray: *mut Il2CppArray, shuffledCharaDataList: *mut Il2CppObject) -> *mut Il2CppObject {
    let chara_vo_ids = &Hachimi::instance().config.load().live_vocals_swap;

    if songId > 0 {
        unsafe {
            if !vocalCharaIdArray.is_null() {
                let len = (*vocalCharaIdArray).max_length as usize;
                let data_ptr = vocalCharaIdArray.add(1) as *mut i32;

                for i in 0..len.min(chara_vo_ids.len()) {
                    if chara_vo_ids[i] != 0 {
                        *data_ptr.add(i) = chara_vo_ids[i];              
                    }
                }
            }

            if !allCharaIdArray.is_null() {
                let len = (*allCharaIdArray).max_length as usize;
                let data_ptr = allCharaIdArray.add(1) as *mut i32;

                for i in 0..len.min(chara_vo_ids.len()) {
                    let new_id = chara_vo_ids[i];
                    if new_id != 0 {
                        *data_ptr.add(i) = new_id;
                    }
                }
            }
        }
    }

    get_orig_fn!(GetSingCharaIdList, GetSingCharaIdListFn)(songId, songPartNumber, allCharaIdArray, vocalCharaIdArray, shuffledCharaDataList)
}

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, "Gallop", LiveUtil);

    let GetSingCharaIdList_addr = get_method_addr(LiveUtil, c"GetSingCharaIdList", 5);
    new_hook!(GetSingCharaIdList_addr, GetSingCharaIdList);

    let InitializeLoadSettings_addr = get_method_addr(LiveUtil, c"InitializeLoadSettings", 3);
    new_hook!(InitializeLoadSettings_addr, InitializeLoadSettings);

    let ChangeLiveSimple_addr = get_method_addr(LiveUtil, c"ChangeLiveSimple", 6);
    new_hook!(ChangeLiveSimple_addr, ChangeLiveSimple);
    
    let ChangeLive_addr = get_method_addr(LiveUtil, c"ChangeLive", 5);
    new_hook!(ChangeLive_addr, ChangeLive);
}

