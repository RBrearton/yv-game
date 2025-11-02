use crate::prelude::*;

/// # Character core
/// A core struct containing some data common to all characters in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterCore {
    pub actor_core: ActorCore,
    pub display_name: DisplayName,
    pub skills: Skills,
    pub equipment: Equipment,
    pub inventory: Inventory,
}

impl HasDisplayName for CharacterCore {
    fn display_name(&self) -> &str {
        self.display_name.as_str()
    }
}

impl Identifiable for CharacterCore {
    fn identity(&self) -> Identity {
        self.actor_core.identity()
    }
}

impl HasPlacement for CharacterCore {
    fn placement(&self) -> &Placement {
        self.actor_core.placement()
    }

    fn placement_mut(&mut self) -> &mut Placement {
        self.actor_core.placement_mut()
    }
}

impl HasSkills for CharacterCore {
    fn skills(&self) -> &Skills {
        &self.skills
    }

    fn skills_mut(&mut self) -> &mut Skills {
        &mut self.skills
    }
}

impl ActorLike for CharacterCore {}

impl HasStats for CharacterCore {
    #[inline]
    fn stats(&self) -> Stats {
        self.equipment.stats()
    }
}

impl CharacterLike for CharacterCore {
    fn meets_requirements(
        &self,
        requirements: &Requirements,
        character_accomplishments: &HashSet<Accomplishment>,
    ) -> bool {
        requirements.are_met_by(
            character_accomplishments,
            &self.skills,
            &self.equipment.stats(),
        )
    }
}
