use crate::{traits::HasDisplayName, well_known_terms};

/// # Stat type
/// All the different kinds of stats that exist in the game.
///
/// While skills are innate to a character and must be gained through experience, stats are things
/// that characterize non-innate efficacy.
/// For example, Strength is a skill that a character can train, while AttackPower is a stat that
/// characterizes how powerful a particular weapon is.
/// The real-life analogue would be that you can make yourself more powerful in melee combat both
/// by training your strength, and by using a better weapon (e.g. a sharper sword, a heavier mace,
/// etc.)
pub enum StatType {
    Armour,
    AttackPower,
    BlockPoints,
    MiningPower,
    Precision,
    RangedAccuracy,
    RangedDamage,
    Speed,
    Warmth,
    WeaponSpeed,
    WoodcuttingPower,
}

impl HasDisplayName for StatType {
    fn display_name(&self) -> &str {
        match self {
            StatType::Armour => well_known_terms::stats::ARMOR,
            StatType::AttackPower => well_known_terms::stats::ATTACK_POWER,
            StatType::BlockPoints => well_known_terms::stats::BLOCK_POINTS,
            StatType::MiningPower => well_known_terms::stats::MINING_POWER,
            StatType::Precision => well_known_terms::stats::PRECISION,
            StatType::RangedAccuracy => well_known_terms::stats::RANGED_ACCURACY,
            StatType::RangedDamage => well_known_terms::stats::RANGED_DAMAGE,
            StatType::Speed => well_known_terms::stats::SPEED,
            StatType::Warmth => well_known_terms::stats::WARMTH,
            StatType::WeaponSpeed => well_known_terms::stats::WEAPON_SPEED,
            StatType::WoodcuttingPower => well_known_terms::stats::WOODCUTTING_POWER,
        }
    }
}
