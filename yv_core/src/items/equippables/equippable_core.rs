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

        Stats::add([
            aug_stats.unwrap_or_default(),
            ench_stats.unwrap_or_default(),
            imb_stats.unwrap_or_default(),
        ])
    }
}

impl Durable for EquippableCore {
    fn durability(&self) -> &Durability {
        &self.durability
    }

    fn durability_mut(&mut self) -> &mut Durability {
        &mut self.durability
    }
}
