mod identity;
mod placement;
mod stat;
mod stat_type;
mod stats;
mod traits;
mod vector_3;
pub mod well_known_terms;

pub mod prelude {
    // I want to use serde Serialize/Deserialize everywhere.
    pub use serde::{Deserialize, Serialize};

    // Every publicly visible type from the crate should be re-exported here.
    pub use crate::{
        identity::Identity, placement::Placement, stat::Stat, stat_type::StatType, stats::Stats,
        traits::*, vector_3::Vector3, well_known_terms,
    };
}
