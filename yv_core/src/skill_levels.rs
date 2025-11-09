use crate::prelude::*;

/// # Skill levels
/// A struct that mirrors the Skills struct, but instead of storing xp, it stores levels.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkillLevels {
    pub strength: u8,
    pub mining: u8,
    pub woodcutting: u8,
    pub magic: u8,
}

impl From<Skills> for SkillLevels {
    fn from(value: Skills) -> Self {
        Self {
            strength: value.strength.level() as u8,
            mining: value.mining.level() as u8,
            woodcutting: value.woodcutting.level() as u8,
            magic: value.magic.level() as u8,
        }
    }
}

impl SkillLevels {
    /// # New
    /// Creates a new set of skill levels with the given levels.
    pub const fn new(strength: u8, mining: u8, woodcutting: u8, magic: u8) -> Self {
        Self {
            strength,
            mining,
            woodcutting,
            magic,
        }
    }
}
