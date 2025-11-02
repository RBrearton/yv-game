use crate::prelude::*;

/// # Inventory
/// A struct representing a character's inventory.
/// This is a simple, fixed size array of items.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    pub items: [Option<Item>; well_known_terms::INVENTORY_SIZE],
}

impl Inventory {
    /// # New
    /// Creates a new inventory containing the provided items.
    pub fn new(items: impl IntoIterator<Item = Item>) -> Self {
        // Create a new inventory.
        let mut inventory = Self::default();
        for (i, item) in items.into_iter().enumerate() {
            // If the inventory is full, break out of the loop.
            if i >= well_known_terms::INVENTORY_SIZE {
                break;
            }

            // Add the item to the inventory.
            inventory.items[i] = Some(item);
        }
        inventory
    }

    /// # Num items
    /// Returns the number of items in the inventory.
    pub fn num_items(&self) -> usize {
        self.items.iter().filter(|item| item.is_some()).count()
    }

    /// # Remove item
    /// Removes the item at the given index.
    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        // Not using the index directly so that we never panic when accessing out of bounds.
        self.items.get_mut(index).and_then(|item| item.take())
    }

    /// # Try add item
    /// Tries to add the given item to the inventory at the first empty index.
    /// Returns true if the item was added successfully, false if the inventory is full.
    pub fn try_add_item(&mut self, item: Item) -> bool {
        let first_empty_index = self.first_empty_index();
        if let Some(index) = first_empty_index {
            return self.try_add_item_at_index(item, index);
        }
        false
    }

    /// # Swap items
    /// Swaps the items at the given indices.
    pub fn swap_items(&mut self, index_1: usize, index_2: usize) {
        self.items.swap(index_1, index_2);
    }

    /// # Try add item at index
    /// Tries to add the given item to the inventory at the given index.
    /// Returns true if the item was added successfully, false if the inventory is full or the index
    /// is out of bounds.
    pub fn try_add_item_at_index(&mut self, item: Item, index: usize) -> bool {
        // Check if the index is out of bounds.
        if index >= well_known_terms::INVENTORY_SIZE {
            return false;
        }

        // Check if the index is already occupied.
        if self.items[index].is_some() {
            return false;
        }

        // Add the item to the inventory.
        self.items[index] = Some(item);
        true
    }

    /// # Get item
    /// Returns a reference to the item at the given index.
    pub fn get_item(&self, index: usize) -> Option<&Item> {
        // Not using the index directly so that we never panic when accessing out of bounds.
        self.items.get(index).and_then(|item| item.as_ref())
    }

    /// # Get item mut
    /// Returns a mutable reference to the item at the given index.
    pub fn get_item_mut(&mut self, index: usize) -> Option<&mut Item> {
        // Not using the index directly so that we never panic when accessing out of bounds.
        self.items.get_mut(index).and_then(|item| item.as_mut())
    }

    /// # Is full
    /// Returns true if the inventory is full.
    pub fn is_full(&self) -> bool {
        self.items.iter().all(|item| item.is_some())
    }

    /// # First empty index
    /// Returns the first empty index in the inventory.
    /// If the inventory is full, returns None.
    pub fn first_empty_index(&self) -> Option<usize> {
        self.items.iter().position(|item| item.is_none())
    }
}
