use crate::prelude::*;

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatType {
    Armour,
    AttackPower,
    BlockPoints,
    CrushingBonus,
    Durability,
    MiningPower,
    Precision,
    RangedAccuracy,
    RangedDamage,
    SlashingBonus,
    Speed,
    StabbingBonus,
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
            StatType::CrushingBonus => well_known_terms::stats::CRUSHING_BONUS,
            StatType::Durability => well_known_terms::stats::DURABILITY,
            StatType::MiningPower => well_known_terms::stats::MINING_POWER,
            StatType::Precision => well_known_terms::stats::PRECISION,
            StatType::RangedAccuracy => well_known_terms::stats::RANGED_ACCURACY,
            StatType::RangedDamage => well_known_terms::stats::RANGED_DAMAGE,
            StatType::SlashingBonus => well_known_terms::stats::SLASHING_BONUS,
            StatType::Speed => well_known_terms::stats::SPEED,
            StatType::StabbingBonus => well_known_terms::stats::STABBING_BONUS,
            StatType::Warmth => well_known_terms::stats::WARMTH,
            StatType::WeaponSpeed => well_known_terms::stats::WEAPON_SPEED,
            StatType::WoodcuttingPower => well_known_terms::stats::WOODCUTTING_POWER,
        }
    }
}
impl Describable for StatType {
    fn description(&self) -> &str {
        match self {
            StatType::Armour => well_known_terms::descriptions::stats::ARMOUR,
            StatType::AttackPower => well_known_terms::descriptions::stats::ATTACK_POWER,
            StatType::BlockPoints => well_known_terms::descriptions::stats::BLOCK_POINTS,
            StatType::CrushingBonus => well_known_terms::descriptions::stats::CRUSHING_BONUS,
            StatType::Durability => well_known_terms::descriptions::stats::DURABILITY,
            StatType::MiningPower => well_known_terms::descriptions::stats::MINING_POWER,
            StatType::Precision => well_known_terms::descriptions::stats::PRECISION,
            StatType::RangedAccuracy => well_known_terms::descriptions::stats::RANGED_ACCURACY,
            StatType::RangedDamage => well_known_terms::descriptions::stats::RANGED_DAMAGE,
            StatType::SlashingBonus => well_known_terms::descriptions::stats::SLASHING_BONUS,
            StatType::Speed => well_known_terms::descriptions::stats::SPEED,
            StatType::StabbingBonus => well_known_terms::descriptions::stats::STABBING_BONUS,
            StatType::Warmth => well_known_terms::descriptions::stats::WARMTH,
            StatType::WeaponSpeed => well_known_terms::descriptions::stats::WEAPON_SPEED,
            StatType::WoodcuttingPower => well_known_terms::descriptions::stats::WOODCUTTING_POWER,
        }
    }
}
