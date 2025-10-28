use crate::prelude::*;

/// # Equippable
/// The trait that must be implemented by anything that can be equipped.
pub trait Equippable: Item {
    /// # Equipment slot
    /// Returns the equipment slot that this equippable item can be equipped in.
    fn equipment_slot(&self) -> EquipmentSlot;

    /// # Requirements
    /// Returns the requirements for equipping this item.
    fn requirements(&self) -> Requirements;
}
