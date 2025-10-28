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
    /// # New from xp
    /// Creates a new set of skills with the given xp values.
    pub fn new_from_xp(strength_xp: i32, mining_xp: i32, woodcutting_xp: i32) -> Self {
        Self {
            strength: Skill::new(SkillType::Strength, strength_xp),
            mining: Skill::new(SkillType::Mining, mining_xp),
            woodcutting: Skill::new(SkillType::Woodcutting, woodcutting_xp),
        }
    }

    /// # New from level
    /// Creates a new set of skills with the given level values.
    pub fn new_from_level(strength_level: i32, mining_level: i32, woodcutting_level: i32) -> Self {
        Self {
            strength: Skill::new_from_level(SkillType::Strength, strength_level),
            mining: Skill::new_from_level(SkillType::Mining, mining_level),
            woodcutting: Skill::new_from_level(SkillType::Woodcutting, woodcutting_level),
        }
    }

    /// # Get skill
    /// Gets the skill with the given type.
    pub fn get_skill(&self, skill_type: SkillType) -> Skill {
        match skill_type {
            SkillType::Strength => self.strength,
            SkillType::Mining => self.mining,
            SkillType::Woodcutting => self.woodcutting,
        }
    }

    /// # Add
    /// Adds two skills together.
    /// This works by adding the underlying xp.
    pub fn add(skills_1: &Skills, skills_2: &Skills) -> Self {
        let str_xp = skills_1.strength.xp + skills_2.strength.xp;
        let mining_xp = skills_1.mining.xp + skills_2.mining.xp;
        let woodcutting_xp = skills_1.woodcutting.xp + skills_2.woodcutting.xp;
        Self {
            strength: Skill::new(SkillType::Strength, str_xp),
            mining: Skill::new(SkillType::Mining, mining_xp),
            woodcutting: Skill::new(SkillType::Woodcutting, woodcutting_xp),
        }
    }
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            strength: Skill::new(SkillType::Strength, 0),
            mining: Skill::new(SkillType::Mining, 0),
            woodcutting: Skill::new(SkillType::Woodcutting, 0),
        }
    }
}
