use crate::prelude::*;

#[derive(Error, Debug)]
pub enum HashGridError {
    #[error("Object with identity `{0}` was not found in the spatial hash grid.")]
    NotFound(Identity),
}
