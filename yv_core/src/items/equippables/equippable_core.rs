use crate::prelude::*;

/// # Equippable core
/// This is a core struct that contains common data that is likely to be shared by all equippables.
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquippableCore {
    pub augmentation: Option<Augmentation>,
    pub enchantment: Option<Enchantment>,
    pub imbuement: Option<Imbuement>,
    pub durability: Durability,
}

impl HasStats for EquippableCore {
    fn stats(&self) -> Stats {
        let aug_stats = self.augmentation.map(|augmentation| augmentation.stats());
        let ench_stats = self.enchantment.map(|enchantment| enchantment.stats());
        let imb_stats = self.imbuement.map(|imbuement| imbuement.stats());
        let stats = Stats::add([
            aug_stats.unwrap_or_default(),
            ench_stats.unwrap_or_default(),
            imb_stats.unwrap_or_default(),
        ]);

        stats
    }
}

impl Durable for EquippableCore {
    fn current_durability(&self) -> u8 {
        self.durability.current_durability()
    }

    fn max_durability(&self) -> u8 {
        self.durability.max_durability()
    }

    fn decrease_durability(&mut self) {
        self.durability.decrease_durability()
    }

    fn restore_durability(&mut self) {
        self.durability.restore_durability()
    }
}
