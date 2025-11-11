pub mod components;
mod prelude;
mod vector_2;

use crate::prelude::*;

fn main() {
    App::new().add_plugins(MinimalPlugins).run();
}
