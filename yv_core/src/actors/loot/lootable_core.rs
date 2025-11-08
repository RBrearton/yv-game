use crate::prelude::*;

/// # Lootable core
/// This is a core struct that sits on top of ActorCore and contains some additional data that is
/// required by all lootable actors.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LootableCore {
    pub actor_core: ActorCore,
    pub loot: Inventory,
    pub looter: Identity,
}

impl HasPlacement for LootableCore {
    fn placement(&self) -> &Placement {
        self.actor_core.placement()
    }

    fn placement_mut(&mut self) -> &mut Placement {
        self.actor_core.placement_mut()
    }
}

impl Identifiable for LootableCore {
    fn identity(&self) -> Identity {
        self.actor_core.identity()
    }
}

impl Lootable for LootableCore {
    fn loot(&self) -> &Inventory {
        &self.loot
    }

    fn looter(&self) -> &Identity {
        &self.looter
    }

    fn loot_mut(&mut self) -> &mut Inventory {
        &mut self.loot
    }

    fn set_looter(&mut self, looter: Identity) {
        self.looter = looter;
    }
}
