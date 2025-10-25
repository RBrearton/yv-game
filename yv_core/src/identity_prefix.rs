use serde::{Deserialize, Serialize};
use std::fmt;

use crate::prelude::*;

/// # Identity prefix
/// The struct containing the prefix for an identity.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityPrefix {
    prefix: [u8; well_known_terms::IDENTITY_PREFIX_CAPACITY],
}
impl IdentityPrefix {
    /// # New identity prefix
    /// Creates a new identity prefix with the given prefix.
    pub fn new(prefix_str: &str) -> Self {
        let prefix_str_len = prefix_str.len();

        // If the prefix string is longer than the capacity, truncate it.
        if prefix_str_len > well_known_terms::IDENTITY_PREFIX_CAPACITY {
            // Recursively call the function with the corrected length.
            return Self::new(&prefix_str[..well_known_terms::IDENTITY_PREFIX_CAPACITY]);
        }

        // Start with a zeroed out prefix.
        let mut prefix = [0u8; well_known_terms::IDENTITY_PREFIX_CAPACITY];

        // Copy the prefix string into the prefix.
        prefix[..prefix_str_len].copy_from_slice(prefix_str.as_bytes());

        Self { prefix }
    }

    /// # Prefix length
    /// Returns the length of the prefix.
    /// The prefix is null terminated, so the length is the index of the first zero byte, if there
    /// is one.
    /// If there isn't one, return the capacity.
    /// Null terminated strings have a bad reputation, but here's we're using them as part of an
    /// exclusively internal data structure - no users will ever touch these strings.
    /// If you're wondering why we're not using a String, it's because all actors, characters, and
    /// entities of any kind will be Identifiable, meaning that they have an identity, and all
    /// identities have a prefix.
    /// If we made the prefix a string, then ALL actors, characters and entities would NEVER be able
    /// to be Copy.
    /// That would mean every single entity of any kind in the game would involve a heap allocation,
    /// which has monumental performance implications.
    pub fn prefix_len(&self) -> usize {
        self.prefix
            .iter()
            .position(|&b| b == 0u8)
            .unwrap_or(well_known_terms::IDENTITY_PREFIX_CAPACITY)
    }
}
impl fmt::Display for IdentityPrefix {
    /// Returns the identity prefix as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the length of the prefix.
        let prefix_len = self.prefix_len();

        // Convert the byte array to a string and write it to the formatter.
        // This from_utf8_lossy function adds that unknown character replacement for invalid
        // byte sequences.
        write!(f, "{}", String::from_utf8_lossy(&self.prefix[..prefix_len]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make sure that the prefix is created correctly when of an appropriate length.
    #[test]
    fn test_new_identity_prefix() {
        let prefix = IdentityPrefix::new("test1");
        assert_eq!(prefix.to_string(), "test1");
    }

    /// Make sure that the prefix is truncated if it is too long.
    #[test]
    fn test_new_identity_prefix_long() {
        let prefix = IdentityPrefix::new("test1234567");
        assert_eq!(prefix.to_string(), "test1234");
    }
}
