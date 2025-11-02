use crate::prelude::*;

/// # Chest
/// An enum containing all chest slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Chest {
    LinenTunic(equippables::chest_slot::LinenTunic),
    LeatherTunic(equippables::chest_slot::LeatherTunic),
}

impl Describable for Chest {
    fn description(&self) -> &str {
        match self {
            Chest::LinenTunic(linen_tunic) => linen_tunic.description(),
            Chest::LeatherTunic(leather_tunic) => leather_tunic.description(),
        }
    }
}
impl HasDisplayName for Chest {
    fn display_name(&self) -> &str {
        match self {
            Chest::LinenTunic(linen_tunic) => linen_tunic.display_name(),
            Chest::LeatherTunic(leather_tunic) => leather_tunic.display_name(),
        }
    }
}

impl HasStats for Chest {
    fn stats(&self) -> Stats {
        match self {
            Chest::LinenTunic(linen_tunic) => linen_tunic.stats(),
            Chest::LeatherTunic(leather_tunic) => leather_tunic.stats(),
        }
    }
}

impl Durable for Chest {
    fn durability(&self) -> &Durability {
        match self {
            Chest::LinenTunic(linen_tunic) => linen_tunic.durability(),
            Chest::LeatherTunic(leather_tunic) => leather_tunic.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Chest::LinenTunic(linen_tunic) => linen_tunic.durability_mut(),
            Chest::LeatherTunic(leather_tunic) => leather_tunic.durability_mut(),
        }
    }
}
