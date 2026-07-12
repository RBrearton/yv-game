use crate::prelude::*;

/// # Character
/// This enum contains a variant for every character in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(HasDisplayName)]
#[delegate(ActorLike)]
pub enum Actor {
    Character(characters::Character),
    EnvironmentActor(environment::EnvironmentActor),
}
