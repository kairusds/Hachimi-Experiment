use crate::il2cpp::types::*;

pub mod SkillUpgradeDescription;

pub fn init(umamusume: *const Il2CppImage) {
    get_class_or_return!(umamusume, Gallop, MasterSkillUpgradeDescription);

    SkillUpgradeDescription::init(MasterSkillUpgradeDescription)
}
