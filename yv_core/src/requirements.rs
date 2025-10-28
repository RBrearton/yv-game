use crate::prelude::*;

/// # Requirements
/// A struct that contains all the requirements for a particular action.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Requirements {
    pub accomplishments: HashSet<Accomplishment>,
    pub skill_levels: HashMap<SkillType, i32>,
    pub stats: HashMap<StatType, f32>,
}

impl Requirements {
    /// # New
    /// Creates a new set of requirements with the given accomplishments, skills, and stats.
    pub fn new(
        accomplishments: HashSet<Accomplishment>,
        skill_levels: HashMap<SkillType, i32>,
        stats: HashMap<StatType, f32>,
    ) -> Self {
        Self {
            accomplishments,
            skill_levels,
            stats,
        }
    }
}
