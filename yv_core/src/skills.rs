use crate::prelude::*;

/// # Skills
/// A collection of skills that a character can have.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skills {
    strength: Skill,
    mining: Skill,
    woodcutting: Skill,
}

impl Skills {
    /// # Get skill
    /// Gets the skill with the given type.
    pub fn get_skill(&self, skill_type: SkillType) -> Skill {
        match skill_type {
            SkillType::Strength => self.strength,
            SkillType::Mining => self.mining,
            SkillType::Woodcutting => self.woodcutting,
        }
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            strength: Skill::new(SkillType::Strength, 0.0),
            mining: Skill::new(SkillType::Mining, 0.0),
            woodcutting: Skill::new(SkillType::Woodcutting, 0.0),
        }
    }
}
