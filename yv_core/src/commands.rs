use crate::prelude::*;

/// # Commands
/// A struct that contains a collection of commands.
/// This is what is returned by a function that needs to mutate the world.
/// This has a fixed size to ensure that we don't need a heap allocation every single time something
/// wants to mutate the world.
/// The simpler solution (returning a Vec<Command>) is very significantly slower if we're making a
/// very large number of commands, and it really isn't the end of the world to just add an option
/// to the commands struct whenever we need to add an additional command.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Commands {
    command_0: Command,
    command_1: Option<Command>,
}

impl Commands {
    /// # New 1
    /// Creates a new commands with one command.
    pub fn new_1(command: Command) -> Self {
        Self {
            command_0: command,
            command_1: None,
        }
    }

    /// # New 2
    /// Creates a new commands with two commands.
    pub fn new_2(command_0: Command, command_1: Command) -> Self {
        Self {
            command_0,
            command_1: Some(command_1),
        }
    }
}

impl IntoIterator for Commands {
    type Item = Command;
    type IntoIter = std::iter::Chain<std::iter::Once<Command>, std::option::IntoIter<Command>>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.command_0).chain(self.command_1)
    }
}

impl From<Command> for Commands {
    fn from(command: Command) -> Self {
        Self::new_1(command)
    }
}

impl From<(Command, Command)> for Commands {
    fn from((command_0, command_1): (Command, Command)) -> Self {
        Self::new_2(command_0, command_1)
    }
}
