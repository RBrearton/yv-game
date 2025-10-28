use crate::prelude::*;

/// # Accomplishment
/// An accomplishment is an achievement in the game.
/// There's an accomplishment for each quest, as well as accomplishments for typical gameplay
/// activities.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Accomplishment {
    QuestStarted(Quest),
    QuestCompleted(Quest),
}
