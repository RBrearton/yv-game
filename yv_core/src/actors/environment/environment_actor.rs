use crate::prelude::*;

/// # Environment actor
/// An environment actor is a type of actor that exists in the game world.
/// Unlike characters, these can't perform actions - they are passive objects that can be
/// interacted with by players and characters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(ActorLike)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
pub enum EnvironmentActor {
    MiningNode(environment::MiningNode),
    Tree(environment::Tree),
}
