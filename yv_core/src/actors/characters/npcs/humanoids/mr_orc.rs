use crate::prelude::*;

/// # Mr. Orc
/// A testing NPC.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasPlacement)]
#[delegate(HasDisplayName)]
#[delegate(HasStats)]
#[delegate(HasSkills)]
#[delegate(ActorLike)]
#[delegate(CharacterLike)]
pub struct MrOrc {
    character_core: characters::CharacterCore,
}

impl Spawnable for MrOrc {
    fn spawn(placement: Placement) -> Self {
        let identity = Identity::new(IdentityPrefix::new("mr_orc"));
        let actor_core = ActorCore {
            identity,
            placement,
        };
        let display_name = DisplayName::new("Mr. Orc");
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
        MrOrc { character_core }
    }
}
