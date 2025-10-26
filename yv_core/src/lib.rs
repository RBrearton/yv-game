mod equipment_slot;
mod identity;
mod identity_prefix;
mod placement;
mod stat;
mod stat_type;
mod stats;
mod traits;
mod vector_3;
pub mod well_known_terms;

pub mod prelude {
    pub use std::fmt;
    pub use uuid::Uuid;

    // I want to use serde Serialize/Deserialize everywhere.
    pub use serde::{Deserialize, Serialize};

    // Every publicly visible type from the crate should be re-exported here.
    pub use crate::{
        equipment_slot::EquipmentSlot, identity::Identity, identity_prefix::IdentityPrefix,
        placement::Placement, stat::Stat, stat_type::StatType, stats::Stats, traits::*,
        vector_3::Vector3, well_known_terms,
    };
}
