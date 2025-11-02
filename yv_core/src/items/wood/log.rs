use crate::prelude::*;

/// # Log
/// A log is a type of wood that can be used to craft items.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Delegate)]
#[delegate(Describable)]
#[delegate(HasDisplayName)]
pub enum Log {
    Birch(wood::BirchLog),
    Oak(wood::OakLog),
    Willow(wood::WillowLog),
}
