use crate::prelude::*;

/// # Command
/// A command is a request to mutate the world.
/// The command pattern is very efficient - it means that we can figure out what commands need to
/// be executed using aggressively parallellised code.
/// We then just have a single mutable pass that iterates over the commands and executes them.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Command {
    DealDamage { target: Identity, damage: u16 },
}
