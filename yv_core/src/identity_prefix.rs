use serde::{Deserialize, Serialize};
use std::fmt;

use crate::prelude::*;

/// # Identity prefix
/// The struct containing the prefix for an identity.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityPrefix {
    prefix: [u8; well_known_terms::MAX_IDENTITY_PREFIX_LEN],
}
impl IdentityPrefix {
    /// # New identity prefix
    /// Creates a new identity prefix with the given prefix.
    pub fn new(prefix_str: &str) -> Self {
        // If the prefix string is longer than MAX_PREFIX_LENGTH, truncate it.
        let prefix_str_len = prefix_str.len();
        if prefix_str_len > well_known_terms::MAX_IDENTITY_PREFIX_LEN {
            // Recursively call the function with the corrected length.
            return Self::new(&prefix_str[..well_known_terms::MAX_IDENTITY_PREFIX_LEN]);
        }

        // Now convert the prefix string to a byte array of the appropriate length.
        let prefix = prefix_str.as_bytes().try_into().unwrap_or_default();
        Self { prefix }
    }
}
impl fmt::Display for IdentityPrefix {
    /// Returns the identity prefix as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert the byte array to a string and write it to the formatter.
        // This from_utf8_lossy function adds that unknown character replacement for invalid
        // byte sequences.
        write!(f, "{}", String::from_utf8_lossy(&self.prefix))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make sure that the prefix is created correctly when of an appropriate length.
    #[test]
    fn test_new_identity_prefix() {
        let prefix = IdentityPrefix::new("test");
        assert_eq!(prefix.to_string(), "test");
    }

    /// Make sure that the prefix is truncated if it is too long.
    #[test]
    fn test_new_identity_prefix_long() {
        let prefix = IdentityPrefix::new("test123456789");
        assert_eq!(prefix.to_string(), "test1234");
    }
}
