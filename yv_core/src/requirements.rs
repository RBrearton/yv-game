use crate::prelude::*;

/// # Requirements
/// A struct that contains all the requirements for a particular action.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Requirements {
    /// We need to be able to define requirements at compile time, which means that there needs to
    /// be a cutoff for the maximum number of accomplishments required to specify an instance of
    /// Requirements.
    pub accomplishments: RequiredAccomplishments,
    pub skill_levels: SkillLevels,
    pub stats: Stats,
}

impl Requirements {
    /// # Minimum
    /// Explicitly creates a new set of requirements with all values set to the minimum values.
    pub const fn minimum() -> Self {
        Self {
            accomplishments: RequiredAccomplishments::empty(),
            skill_levels: SkillLevels::zeroes(),
            stats: Stats::empty(),
        }
    }

    /// # New
    /// Creates a new set of requirements with the given accomplishments, skills, and stats.
    pub const fn new(
        accomplishments: RequiredAccomplishments,
        skill_levels: SkillLevels,
        stats: Stats,
    ) -> Self {
        Self {
            accomplishments,
            skill_levels,
            stats,
        }
    }

    /// # Are met by
    /// Returns whether the given requirements are met by the provided accomplishments, skills and
    /// stats.
    pub fn are_met_by(
        &self,
        character_accomplishments: &HashSet<Accomplishment>,
        character_skills: &Skills,
        character_stats: &Stats,
    ) -> bool {
        // Start by checking the accomplishments.
        for required_accomplishment in self.accomplishments.into_iter() {
            if !character_accomplishments.contains(&required_accomplishment) {
                return false;
            }
        }

        // All of the character's skill must be greater than or equal to the required skill levels.
        let character_skill_levels: SkillLevels = character_skills.into();
        if !character_skill_levels.are_all_greater_than_or_equal_to(&self.skill_levels) {
            return false;
        }

        // All of the characters stats must be greater than or equal to the required stats.
        if !character_stats.are_all_greater_than_or_equal_to(&self.stats) {
            return false;
        }

        // If execution reaches here, we've passed all checks and the requirements are met.
        true
    }
}
