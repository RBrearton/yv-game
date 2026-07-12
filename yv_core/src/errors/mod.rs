//! # Errors
//! This module contains the definition of all the errors used in the game.
//! We use thiserror to make error definitions as ergonomic as possible.

mod hash_grid_error;

pub use hash_grid_error::HashGridError;
