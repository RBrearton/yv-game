mod equipment_slot;
mod identity;
mod identity_prefix;
mod placement;
mod spatial_hash_grid;
mod stat;
mod stat_type;
mod stats;
mod traits;
mod vector_2;
pub mod well_known_terms;

pub mod prelude {
    pub use std::fmt;
    pub use uuid::Uuid;

    // I want to use serde Serialize/Deserialize everywhere.
    pub use serde::{Deserialize, Serialize};

    // I want to use hashbrown hashmaps and hashsets throughout the crate.
    pub use hashbrown::{HashMap, HashSet};

    // Every publicly visible type from the crate should be re-exported here.
    pub use crate::{
        equipment_slot::EquipmentSlot, identity::Identity, identity_prefix::IdentityPrefix,
        placement::Placement, spatial_hash_grid::SpatialHashGrid, stat::Stat, stat_type::StatType,
        stats::Stats, traits::*, vector_2::Vector2, well_known_terms,
    };
}
