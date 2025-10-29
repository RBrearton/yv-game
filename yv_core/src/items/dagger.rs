use crate::prelude::*;

const STONE_DAGGER_DESCRIPTION: &str = "A stone dagger.";
const IRON_DAGGER_DESCRIPTION: &str = "An iron dagger.";

const STONE_DAGGER_DISPLAY_NAME: &str = "Stone dagger";
const IRON_DAGGER_DISPLAY_NAME: &str = "Iron dagger";

/// # Dagger
/// An enum containing all dagger slot items in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Dagger {
    StoneDagger,
    IronDagger,
}

impl Describable for Dagger {
    fn description(&self) -> &str {
        match self {
            Dagger::StoneDagger => STONE_DAGGER_DESCRIPTION,
            Dagger::IronDagger => IRON_DAGGER_DESCRIPTION,
        }
    }
}
impl HasDisplayName for Dagger {
    fn display_name(&self) -> &str {
        match self {
            Dagger::StoneDagger => STONE_DAGGER_DISPLAY_NAME,
            Dagger::IronDagger => IRON_DAGGER_DISPLAY_NAME,
        }
    }
}
