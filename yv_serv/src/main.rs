pub mod components;
pub mod well_known_terms;

mod identity_prefix;
mod prelude;
mod vector_2;

use crate::prelude::*;

fn main() {
    App::new().add_plugins(MinimalPlugins).run();
}
