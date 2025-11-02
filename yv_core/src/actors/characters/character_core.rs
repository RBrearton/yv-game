use crate::prelude::*;

/// # Character core
/// A core struct containing some data common to all characters in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterCore {
    pub actor_core: ActorCore,
    pub skills: Skills,
    pub equipment: Equipment,
    pub inventory: Inventory,
}

impl HasPlacement for CharacterCore {
    fn placement(&self) -> &Placement {
        self.actor_core.placement()
    }

    fn placement_mut(&mut self) -> &mut Placement {
        self.actor_core.placement_mut()
    }
}

impl HasDisplayName for CharacterCore {
    fn display_name(&self) -> &str {
        self.actor_core.display_name()
    }
}

impl Identifiable for CharacterCore {
    fn identity(&self) -> Identity {
        self.actor_core.identity()
    }
}

impl ActorLike for CharacterCore {}

impl HasSkills for CharacterCore {
    fn skills(&self) -> &Skills {
        &self.skills
    }

    fn skills_mut(&mut self) -> &mut Skills {
        &mut self.skills
    }
}

impl HasStats for CharacterCore {
    fn stats(&self) -> Stats {
        self.equipment.stats()
    }
}
