use crate::prelude::*;

/// # Skill
/// A skill is a particular aspect of a character's ability.
/// It is used to calculate the overall effectiveness of a character in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
    pub skill_type: SkillType,
    pub value: f32,
}

impl Skill {
    /// # New skill
    /// Creates a new skill with the given type and value.
    pub fn new(skill_type: SkillType, value: f32) -> Self {
        Self { skill_type, value }
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
