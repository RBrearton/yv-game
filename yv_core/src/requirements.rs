use crate::prelude::*;

/// # Requirements
/// A struct that contains all the requirements for a particular action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Requirements {
    pub accomplishments: HashSet<Accomplishment>,
    pub skill_levels: HashMap<SkillType, i32>,
    pub stats: HashMap<StatType, i16>,
}

impl Requirements {
    /// # New
    /// Creates a new set of requirements with the given accomplishments, skills, and stats.
    pub fn new(
        accomplishments: HashSet<Accomplishment>,
        skill_levels: HashMap<SkillType, i32>,
        stats: HashMap<StatType, i16>,
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
        for required_accomplishment in &self.accomplishments {
            if !character_accomplishments.contains(required_accomplishment) {
                return false;
            }
        }

        // Now check the skills.
        for (&skill_type, &required_level) in &self.skill_levels {
            let skill_level = character_skills.get_skill_level(skill_type);
            if skill_level < required_level {
                return false;
            }
        }

        // Now check the stats.
        for (&stat_type, &required_value) in &self.stats {
            let stat_value = character_stats.get_stat(stat_type).value;
            if stat_value < required_value {
                return false;
            }
        }

        true
    }
}
