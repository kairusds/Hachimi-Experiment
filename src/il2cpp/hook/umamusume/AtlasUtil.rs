use crate::il2cpp::{ext::Il2CppStringExt, symbols::{get_method_addr, get_method_overload_addr}, types::*};
use std::sync::atomic::Ordering;
use super::GameSystem::GAME_INITIALIZED;

static mut LAST_ATLAS_PTR: *mut Il2CppString = std::ptr::null_mut();
static mut LAST_SPRITE_PTR: *mut Il2CppString = std::ptr::null_mut();
static mut LAST_TYPE: i32 = -1;

#[derive(Default, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum TargetAtlasType {
    #[default] None = 0,
    PreIn = 1,
    Common = 2,
    Home = 3,
    Gacha = 4,
    Live = 5,
    Race = 6,
    RaceCommon = 7,
    Story = 8,
    Umamusume = 9,
    Single = 10,
    Single2 = 11,
    Option = 12,
    Note = 13,
    Circle = 14,
    RaceOrder = 15,
    Rank = 16,
    Trophy = 17,
    Directory = 18,
    TeamStadium = 19,
    Daily = 20,
    Shop = 21,
    Minigame = 22,
    SingleStart = 23,
    SingleResult = 24,
    SingleCommon = 25,
    StatusRank = 26,
    ChampionsMeeting = 27,
    StoryEvent = 28,
    RoomMatch = 29,
    ProfileCard = 30,
    Mission = 31,
    TransferEvent = 32,
    SingleModeScenarioTeamRace = 33,
    SingleModeScenarioFree = 34,
    PracticeRace = 35,
    TrainingChallenge = 36,
    ChallengeMatch = 37,
    FanRaid = 38,
    TeamBuilding = 39,
    SingleModeScenarioLive = 40,
    Campaign = 41,
    PhotoStudio = 42,
    MapEvent = 43,
    SingleModeScenarioVenus = 44,
    SingleModeScenarioVenusSpiritIcon = 45,
    FactorResearch = 46,
    Heroes = 47,
    SingleModeScenarioArc = 48,
    SingleModeScenarioSport = 49,
    SingleModeScenarioCook = 50,
    SingleModeScenarioMecha = 51,
    Outgame = 52,
    RatingRace = 53,
    TrainingReport = 54,
    RaceFitAssistance = 55,
    SingleModeScenarioLegend = 56,
    UltimateRace = 57,
    SingleModeScenarioPioneer = 58,
    SingleModeScenarioPioneerFacilityIcon = 59,
    WindowsPreIn = 60,
    ScheduleBook = 61,
    UmatubeRaid = 62,
    Walking = 63,
    SingleModeScenarioOnsen = 64,
    Max = 65,
}

type GetSpriteByNameFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    /*if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasName != unsafe { LAST_ATLAS_PTR } || spriteName != unsafe { LAST_SPRITE_PTR } {
            let atlas = unsafe { atlasName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            let sprite = unsafe { spriteName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());

            info!("GetSpriteByName (string, string): {}, {}", atlas, sprite);
            unsafe {
                LAST_ATLAS_PTR = atlasName;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }*/
    get_orig_fn!(GetSpriteByName, GetSpriteByNameFn)(atlasName, spriteName)
}

type GetSpriteByName1Fn = extern "C" fn(atlasType: TargetAtlasType, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteByName1(atlasType: TargetAtlasType, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    /* if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasType != unsafe { LAST_TYPE } || spriteName != unsafe { LAST_SPRITE_PTR } {
            let sprite = unsafe { spriteName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            info!("GetSpriteByName (i32, string): {}, {}", atlasType, sprite);
            unsafe {
                LAST_TYPE = atlasType;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    } */
    get_orig_fn!(GetSpriteByName1, GetSpriteByName1Fn)(atlasType, spriteName)
}

type GetSpriteFromNameSubFn = extern "C" fn(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject;
extern "C" fn GetSpriteFromNameSub(atlasName: *mut Il2CppString, spriteName: *mut Il2CppString) -> *mut Il2CppObject {
    /*
    if GAME_INITIALIZED.load(Ordering::Relaxed) {
        if atlasName != unsafe { LAST_ATLAS_PTR } || spriteName != unsafe { LAST_SPRITE_PTR } {
            let atlas = unsafe { atlasName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            let sprite = unsafe { spriteName.as_ref() }.map_or("empty".to_string(), |s| unsafe { s.as_utf16str() }.to_string());
            info!("GetSpriteFromNameSubFn (string, string): {}, {}", atlas, sprite);
            unsafe {
                LAST_ATLAS_PTR = atlasName;
                LAST_SPRITE_PTR = spriteName;
            }
        }
    }*/
    get_orig_fn!(GetSpriteFromNameSub, GetSpriteFromNameSubFn)(atlasName, spriteName)
}

// public static Sprite GetSpriteByName(String atlasName, String spriteName) { }
// public static Sprite GetSpriteByName(TargetAtlasType atlasType, String spriteName) { }
// private static Sprite GetSpriteFromNameSub(String atlasName, String spriteName) { }
	
    
pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, AtlasUtil);

    let GetSpriteByName_addr = get_method_overload_addr(AtlasUtil, "GetSpriteByName",
        &[Il2CppTypeEnum_IL2CPP_TYPE_STRING, Il2CppTypeEnum_IL2CPP_TYPE_STRING]);
    new_hook!(GetSpriteByName_addr, GetSpriteByName);

    /*
    let GetSpriteByName1_addr = get_method_overload_addr(AtlasUtil, "GetSpriteByName",
        &[Il2CppTypeEnum_IL2CPP_TYPE_VALUETYPE, Il2CppTypeEnum_IL2CPP_TYPE_STRING]);
    new_hook!(GetSpriteByName1_addr, GetSpriteByName1);

    let GetSpriteFromNameSub_addr = get_method_addr(AtlasUtil, c"GetSpriteFromNameSub", 2);
    new_hook!(GetSpriteFromNameSub_addr, GetSpriteFromNameSub);
    */
}
