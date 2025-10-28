use crate::prelude::*;

/// # Has skills
/// The trait that must be implemented by anything that has skills.
pub trait HasSkills {
    /// # Skills
    /// Returns a reference to the skills of the object.
    fn skills(&self) -> &Skills;

    /// # Get skill
    /// Gets the skill with the given type.
    fn get_skill(&self, skill_type: SkillType) -> Skill;

    /// # Get skill level
    /// Gets the level of the skill with the given type.
    fn get_skill_level(&self, skill_type: SkillType) -> i32;

    /// # Get skill xp
    /// Gets the xp of the skill with the given type.
    fn get_skill_xp(&self, skill_type: SkillType) -> i32;

    /// # Skills mut
    /// Returns a mutable reference to the skills of the object.
    fn skills_mut(&mut self) -> &mut Skills;
}
