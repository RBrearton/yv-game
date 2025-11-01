use crate::prelude::*;

/// # Belt
/// An enum containing all belt slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Belt {
    SimpleLeatherBelt(belts::SimpleLeatherBelt),
}

impl Describable for Belt {
    fn description(&self) -> &str {
        match self {
            Belt::SimpleLeatherBelt(simple_leather_belt) => simple_leather_belt.description(),
        }
    }
}

impl HasDisplayName for Belt {
    fn display_name(&self) -> &str {
        match self {
            Belt::SimpleLeatherBelt(simple_leather_belt) => simple_leather_belt.display_name(),
        }
    }
}

impl HasStats for Belt {
    fn stats(&self) -> &Stats {
        match self {
            Belt::SimpleLeatherBelt(simple_leather_belt) => simple_leather_belt.stats(),
        }
    }
}
