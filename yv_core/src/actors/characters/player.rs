use crate::prelude::*;

/// # Player
/// A player that spawns as a member of the human race.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(HasDisplayName)]
#[delegate(HasStats)]
#[delegate(HasSkills)]
#[delegate(ActorLike)]
#[delegate(CharacterLike)]
pub struct Player {
    character_core: characters::CharacterCore,
}

impl Spawnable for Player {
    fn spawn(placement: Placement) -> Self {
        let identity = Identity::new(IdentityPrefix::new("player"));
        let actor_core = ActorCore {
            identity,
            placement,
        };
        let display_name = DisplayName::new("Player");
        let skills = Skills::default();
        let equipment = Equipment::default();
        let inventory = Inventory::default();
        let character_core = characters::CharacterCore {
            actor_core,
            display_name,
            skills,
            equipment,
            inventory,
        };
        Player { character_core }
    }
}
