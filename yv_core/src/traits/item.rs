use crate::prelude::*;

pub trait Item:
    Ownable
    + Describable
    + HasDisplayName
    + HasStats
    + Massive
    + Copy
    + Clone
    + Serialize
    + for<'a> Deserialize<'a>
{
}
