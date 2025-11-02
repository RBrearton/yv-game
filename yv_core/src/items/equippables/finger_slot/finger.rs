use crate::prelude::*;

/// # Finger
/// An enum containing all finger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Finger {
    GoldRing(equippables::finger_slot::GoldRing),
}

impl Describable for Finger {
    fn description(&self) -> &str {
        match self {
            Finger::GoldRing(gold_ring) => gold_ring.description(),
        }
    }
}

impl HasDisplayName for Finger {
    fn display_name(&self) -> &str {
        match self {
            Finger::GoldRing(gold_ring) => gold_ring.display_name(),
        }
    }
}

impl HasStats for Finger {
    fn stats(&self) -> Stats {
        match self {
            Finger::GoldRing(gold_ring) => gold_ring.stats(),
        }
    }
}

impl Durable for Finger {
    fn durability(&self) -> &Durability {
        match self {
            Finger::GoldRing(gold_ring) => gold_ring.durability(),
        }
    }

    fn durability_mut(&mut self) -> &mut Durability {
        match self {
            Finger::GoldRing(gold_ring) => gold_ring.durability_mut(),
        }
    }
}
