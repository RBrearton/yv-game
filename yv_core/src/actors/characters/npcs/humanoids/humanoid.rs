use crate::prelude::*;

/// # Humanoid
/// This enum contains a variant for every humanoid NPC in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(HasDisplayName)]
#[delegate(HasStats)]
#[delegate(HasSkills)]
#[delegate(ActorLike)]
#[delegate(CharacterLike)]
pub enum Humanoid {
    MrOrc(characters::npcs::humanoids::MrOrc),
}
