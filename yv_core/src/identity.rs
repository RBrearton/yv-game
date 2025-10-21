use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// # Identity
/// A unique identifier for an object.
/// Instances of identity are associated both with a prefix and a Uuid identifier.
/// The prefix comes from a strong preference of mine - I'd much rather have something like:
/// orc-1234-1234-1234 than just 1234-1234-1234
/// The `orc` part really makes investigating logs etc. much more intuitive.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    prefix: String,
    unique_id: Uuid,
}
impl Identity {
    /// # New identity
    /// Creates a new identity with the given prefix.
    /// This generates a new unique identifier for the identity.
    pub fn new(prefix: String) -> Self {
        let unique_id = Uuid::new_v4();
        Self { prefix, unique_id }
    }
}

impl fmt::Display for Identity {
    /// Returns the identity as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.prefix, self.unique_id)
    }
}
