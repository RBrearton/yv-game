use crate::prelude::*;

/// # Skill levels
/// A struct that mirrors the Skills struct, but instead of storing xp, it stores levels.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
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

impl From<&Skills> for SkillLevels {
    fn from(value: &Skills) -> Self {
        Self {
            strength: value.strength.level() as u8,
            mining: value.mining.level() as u8,
            woodcutting: value.woodcutting.level() as u8,
            magic: value.magic.level() as u8,
        }
    }
}

impl IntoIterator for SkillLevels {
    type Item = (SkillType, u8);
    type IntoIter = std::array::IntoIter<(SkillType, u8), 4>;

    fn into_iter(self) -> Self::IntoIter {
        let arr = [
            (SkillType::Strength, self.strength),
            (SkillType::Mining, self.mining),
            (SkillType::Woodcutting, self.woodcutting),
            (SkillType::Magic, self.magic),
        ];
        arr.into_iter()
    }
}

impl SkillLevels {
    /// # Zeroes
    /// Explicitly creates a new set of skill levels with all values set to 0.
    pub const fn zeroes() -> Self {
        Self {
            strength: 0,
            mining: 0,
            woodcutting: 0,
            magic: 0,
        }
    }

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

    /// # Are all greater than or equal to
    /// Returns whether all the skill levels are greater than or equal to the given level.
    pub fn are_all_greater_than_or_equal_to(&self, skill_levels: &SkillLevels) -> bool {
        self.strength >= skill_levels.strength
            && self.mining >= skill_levels.mining
            && self.woodcutting >= skill_levels.woodcutting
            && self.magic >= skill_levels.magic
    }
}
