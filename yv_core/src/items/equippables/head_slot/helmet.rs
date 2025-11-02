use crate::prelude::*;

/// # Helmet
/// An enum containing all helmet slot armour items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Helmet {
    IronSkullcap(equippables::head_slot::IronSkullcap),
}

impl Describable for Helmet {
    fn description(&self) -> &str {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.description(),
        }
    }
}
impl HasDisplayName for Helmet {
    fn display_name(&self) -> &str {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.display_name(),
        }
    }
}

impl HasStats for Helmet {
    fn stats(&self) -> Stats {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.stats(),
        }
    }
}

impl Durable for Helmet {
    fn current_durability(&self) -> u8 {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.current_durability(),
        }
    }

    fn max_durability(&self) -> u8 {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.max_durability(),
        }
    }

    fn decrease_durability(&mut self) {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.decrease_durability(),
        }
    }

    fn restore_durability(&mut self) {
        match self {
            Helmet::IronSkullcap(iron_skullcap) => iron_skullcap.restore_durability(),
        }
    }
}
