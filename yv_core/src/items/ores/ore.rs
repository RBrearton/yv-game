use crate::prelude::*;

/// # Ore
/// An enum where each variant is a different type of ore.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Describable)]
#[delegate(HasDisplayName)]
pub enum Ore {
    Copper(ores::CopperOre),
    Iron(ores::IronOre),
}
