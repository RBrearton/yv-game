use crate::prelude::*;

/// # NPC
/// This enum contains a variant for every NPC in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(HasDisplayName)]
#[delegate(HasStats)]
#[delegate(HasSkills)]
#[delegate(ActorLike)]
#[delegate(CharacterLike)]
pub enum Npc {
    Humanoid(characters::npcs::humanoids::Humanoid),
}
