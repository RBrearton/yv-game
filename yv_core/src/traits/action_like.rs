use crate::prelude::*;

/// # Action like
/// The trait that must be implemented by anything that can be acted upon.
#[delegatable_trait]
pub trait ActionLike: Identifiable + Describable {
    /// # Range
    /// Returns the range of the action.
    fn range(&self) -> f32;

    /// # Cooldown
    /// Returns the cooldown of the action.
    fn cooldown(&self) -> Duration;

    /// # Cast time
    /// The time it takes to cast the action.
    /// If this is None, then the action is to be treated as an instant cast.
    fn cast_time(&self) -> Option<Duration>;

    /// # Target
    /// The identity of the target of the action.
    /// If this is None, then the action is a self-targeting action.
    fn target(&self) -> Option<&Identity>;

    /// # Performer
    /// The identity of the performer of the action.
    fn performer(&self) -> &Identity;

    /// # On completion
    /// The method called when the action finishes.
    fn on_completion(&self, world: &mut World);
}
