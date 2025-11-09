use crate::prelude::*;

/// # Action
/// An enum containing a variant for each type of action in the game.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Identifiable)]
#[delegate(HasDisplayName)]
#[delegate(Describable)]
#[delegate(ActionLike)]
#[delegate(HasRequirements)]
pub enum Action {
    ArcaneBolt(actions::ArcaneBolt),
}
