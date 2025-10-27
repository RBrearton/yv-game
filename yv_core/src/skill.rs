use crate::prelude::*;

/// # Skill
/// A skill is a particular aspect of a character's ability.
/// It is used to calculate the overall effectiveness of a character in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub skill_type: SkillType,
    pub xp: i32,
}

impl Skill {
    /// # New skill
    /// Creates a new skill with the given type and value.
    pub fn new(skill_type: SkillType, xp: i32) -> Self {
        Self { skill_type, xp }
    }

    /// # New from level
    /// Creates a new skill with the given type and level.
    pub fn new_from_level(skill_type: SkillType, level: i32) -> Self {
        Self {
            skill_type,
            xp: Self::xp_required(level),
        }
    }

    /// # Level
    /// Calculates the level of the skill based on the current xp.
    /// The maximum level is capped at 50.
    pub fn level(&self) -> i32 {
        // This was found by spending a bunch of time playing around.
        // Cubic scaling seems to be a good fit for the game.
        let level = (self.xp as f32 / 64.0).cbrt() + 1.0;
        let level = level.floor() as i32;
        // Cap the level at 50.
        level.min(50)
    }

    /// # XP to next level
    /// Calculates the amount of xp required to reach the next level.
    pub fn xp_to_next_level(&self) -> i32 {
        let next_level = self.level() + 1;
        Self::xp_required(next_level) - self.xp
    }

    /// # XP in current level
    /// Calculates the amount of XP we've gained in the current level.
    pub fn xp_in_current_level(&self) -> i32 {
        self.xp - Self::xp_required(self.level())
    }

    /// # XP required
    /// Calculates the total amount of xp required to reach a certain level.
    pub fn xp_required(level: i32) -> i32 {
        // Avoid negative values of xp.
        if level < 1 {
            return 0;
        }

        // The inverse of the formula used to calculate the level.
        let xp = (level as f32 - 1.0).powf(3.0) * 64.0;
        xp.round() as i32
    }
}

impl HasDisplayName for Skill {
    fn display_name(&self) -> &str {
        self.skill_type.display_name()
    }
}
impl Describable for Skill {
    fn description(&self) -> &str {
        self.skill_type.description()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// # Test level calculation
    /// Make sure that the level calculation has the scaling that we desire.
    /// This test pretty much characterizes the behaviour of the function.
    #[test]
    fn test_level() {
        assert_eq!(Skill::new(SkillType::Woodcutting, 0).level(), 1);
        assert_eq!(Skill::new(SkillType::Woodcutting, 1).level(), 1);
        assert_eq!(Skill::new(SkillType::Woodcutting, 64).level(), 2);
        assert_eq!(Skill::new(SkillType::Woodcutting, 1000).level(), 3);
        assert_eq!(Skill::new(SkillType::Woodcutting, 10000).level(), 6);
        assert_eq!(Skill::new(SkillType::Woodcutting, 100000).level(), 12);
        assert_eq!(Skill::new(SkillType::Woodcutting, 1_000_000).level(), 26);
        assert_eq!(Skill::new(SkillType::Woodcutting, 7_800_000).level(), 50);
        assert_eq!(Skill::new(SkillType::Woodcutting, 10_000_000).level(), 50);
        assert_eq!(Skill::new(SkillType::Woodcutting, 100_000_000).level(), 50);
    }

    /// # Test XP required calculation
    /// Make sure that the XP required calculation has the scaling that we desire.
    /// This test pretty much characterizes the behaviour of the function.
    #[test]
    fn test_xp_required() {
        assert_eq!(Skill::xp_required(0), 0);
        assert_eq!(Skill::xp_required(1), 0);
        assert_eq!(Skill::xp_required(2), 64);
        assert_eq!(Skill::xp_required(3), 512);
        assert_eq!(Skill::xp_required(6), 8000);
        assert_eq!(Skill::xp_required(12), 85_184);
        assert_eq!(Skill::xp_required(26), 1_000_000);
        assert_eq!(Skill::xp_required(50), 7_529_536);
    }
}
