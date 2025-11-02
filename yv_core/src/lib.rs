mod accomplishment;
mod actors;
mod augmentation;
mod display_name;
mod durability;
mod enchantment;
mod equipment;
mod identity;
mod identity_prefix;
mod imbuement;
mod inventory;
mod items;
mod placement;
mod quest;
mod requirements;
mod skill;
mod skill_type;
mod skills;
mod spatial_hash_grid;
mod stat;
mod stat_type;
mod stats;
mod traits;
mod vector_2;
pub mod well_known_terms;
mod world;

pub mod prelude {
    pub use std::fmt;
    pub use uuid::Uuid;

    // I want to use serde Serialize/Deserialize everywhere.
    pub use serde::{Deserialize, Serialize};

    // I want to use hashbrown hashmaps and hashsets throughout the crate.
    pub use hashbrown::{HashMap, HashSet};

    // Every publicly visible type from the crate should be re-exported here.
    pub use crate::{
        accomplishment::Accomplishment, actors::*, augmentation::Augmentation,
        display_name::DisplayName, durability::Durability, enchantment::Enchantment,
        equipment::Equipment, identity::Identity, identity_prefix::IdentityPrefix,
        imbuement::Imbuement, inventory::Inventory, items::*, placement::Placement, quest::Quest,
        requirements::Requirements, skill::Skill, skill_type::SkillType, skills::Skills,
        spatial_hash_grid::SpatialHashGrid, stat::Stat, stat_type::StatType, stats::Stats,
        traits::*, vector_2::Vector2, well_known_terms, world::World,
    };
}
