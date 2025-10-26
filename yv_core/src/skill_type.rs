use crate::prelude::*;

/// # Skill type
/// An enum representing the different types of skills that exist in the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillType {
    Strength,
    Mining,
    Woodcutting,
}

impl HasDisplayName for SkillType {
    fn display_name(&self) -> &str {
        match self {
            SkillType::Strength => well_known_terms::skills::STRENGTH,
            SkillType::Mining => well_known_terms::skills::MINING,
            SkillType::Woodcutting => well_known_terms::skills::WOODCUTTING,
        }
    }
}

impl Describable for SkillType {
    fn description(&self) -> &str {
        match self {
            SkillType::Strength => well_known_terms::descriptions::skills::STRENGTH,
            SkillType::Mining => well_known_terms::descriptions::skills::MINING,
            SkillType::Woodcutting => well_known_terms::descriptions::skills::WOODCUTTING,
        }
    }
}
