use crate::prelude::*;

/// # Display name
/// This is a struct that we use to store display names for objects.
/// Rather importantly, this is a fixed size array of characters so that it can be Copy.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisplayName {
    name: [u8; well_known_terms::DISPLAY_NAME_CAPACITY],
    len: usize,
}
impl DisplayName {
    /// # New display name
    /// Creates a new display name with the given name.
    pub fn new(name: &str) -> Self {
        let mut name_bytes = [0u8; well_known_terms::DISPLAY_NAME_CAPACITY];
        let bytes = name.as_bytes();
        let len = bytes.len().min(well_known_terms::DISPLAY_NAME_CAPACITY);
        name_bytes[..len].copy_from_slice(&bytes[..len]);
        Self {
            name: name_bytes,
            len,
        }
    }

    /// # As str
    /// Returns the display name as a string.
    pub fn as_str(&self) -> &str {
        self.display_name()
    }

    /// # Display name length
    /// Returns the length of the display name.
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl HasDisplayName for DisplayName {
    fn display_name(&self) -> &str {
        std::str::from_utf8(&self.name[..self.len]).unwrap_or("IF YOU SEE THIS... OH GOD")
    }
}

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.name[..self.len]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Make sure that the display name is created correctly.
    #[test]
    fn test_new_display_name() {
        let display_name = DisplayName::new("test");
        assert_eq!(display_name.to_string(), "test");
        assert_eq!(display_name.len(), 4);
    }

    /// Make sure that the display name is truncated if it is too long.
    #[test]
    fn test_new_display_name_long() {
        let display_name = DisplayName::new("1234567890123456789");
        assert_eq!(display_name.to_string(), "1234567890123456");
        assert_eq!(display_name.len(), 16);
    }
}
