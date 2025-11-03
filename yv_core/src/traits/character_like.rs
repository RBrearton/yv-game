use crate::prelude::*;

/// # Character like
/// The trait that needs to be implemented by anything that wants to be treated as a character in
/// the game.
/// This includes all player characters, as well as all non-player characters.
#[delegatable_trait]
pub trait CharacterLike: ActorLike + HasSkills + HasStats {
    /// # Meets requirements
    /// Returns whether this character meets the given requirements.
    /// Note that the character's accomplishments are passed in separately.
    /// As accomplishments can be very large, CharacterLike implementations aren't expected to
    /// store them themselves.
    fn meets_requirements(
        &self,
        requirements: &Requirements,
        character_accomplishments: &HashSet<Accomplishment>,
    ) -> bool;
}
