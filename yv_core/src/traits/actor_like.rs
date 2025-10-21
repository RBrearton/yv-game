use crate::traits::{HasPlacement, Identifiable};

pub trait ActorLike: HasPlacement + Identifiable {}
