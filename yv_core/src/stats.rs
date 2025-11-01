use crate::prelude::*;

/// # Stats
/// A struct that contains all possible stats.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub armour: Stat,
    pub attack_power: Stat,
    pub block_points: Stat,
    pub mining_power: Stat,
    pub precision: Stat,
    pub ranged_accuracy: Stat,
    pub ranged_damage: Stat,
    pub speed: Stat,
    pub warmth: Stat,
    pub weapon_speed: Stat,
    pub woodcutting_power: Stat,
}

impl Default for Stats {
    /// # Default
    /// Creates a new set of stats with all values set to 0.
    /// Under the hood, this is identical to calling `Stats::empty()`, but often leads to more
    /// idiomatic rust.
    fn default() -> Self {
        Self::empty()
    }
}

impl Stats {
    /// # New empty
    /// Explicitly creates a new set of stats with all values set to 0.
    pub fn empty() -> Self {
        Self {
            armour: Stat::new(StatType::Armour, 0),
            attack_power: Stat::new(StatType::AttackPower, 0),
            block_points: Stat::new(StatType::BlockPoints, 0),
            mining_power: Stat::new(StatType::MiningPower, 0),
            precision: Stat::new(StatType::Precision, 0),
            ranged_accuracy: Stat::new(StatType::RangedAccuracy, 0),
            ranged_damage: Stat::new(StatType::RangedDamage, 0),
            speed: Stat::new(StatType::Speed, 0),
            warmth: Stat::new(StatType::Warmth, 0),
            weapon_speed: Stat::new(StatType::WeaponSpeed, 0),
            woodcutting_power: Stat::new(StatType::WoodcuttingPower, 0),
        }
    }

    /// # Get stat
    /// Gets the stat with the given type.
    pub fn get_stat(&self, stat_type: StatType) -> Stat {
        match stat_type {
            StatType::Armour => self.armour,
            StatType::AttackPower => self.attack_power,
            StatType::BlockPoints => self.block_points,
            StatType::MiningPower => self.mining_power,
            StatType::Precision => self.precision,
            StatType::RangedAccuracy => self.ranged_accuracy,
            StatType::RangedDamage => self.ranged_damage,
            StatType::Speed => self.speed,
            StatType::Warmth => self.warmth,
            StatType::WeaponSpeed => self.weapon_speed,
            StatType::WoodcuttingPower => self.woodcutting_power,
        }
    }

    /// # Set stat
    /// Sets the stat with the given type to the given value.
    pub fn set_stat(&mut self, stat_type: StatType, value: i32) {
        match stat_type {
            StatType::Armour => self.armour = Stat::new(StatType::Armour, value),
            StatType::AttackPower => self.attack_power = Stat::new(StatType::AttackPower, value),
            StatType::BlockPoints => self.block_points = Stat::new(StatType::BlockPoints, value),
            StatType::MiningPower => self.mining_power = Stat::new(StatType::MiningPower, value),
            StatType::Precision => self.precision = Stat::new(StatType::Precision, value),
            StatType::RangedAccuracy => {
                self.ranged_accuracy = Stat::new(StatType::RangedAccuracy, value)
            }
            StatType::RangedDamage => self.ranged_damage = Stat::new(StatType::RangedDamage, value),
            StatType::Speed => self.speed = Stat::new(StatType::Speed, value),
            StatType::Warmth => self.warmth = Stat::new(StatType::Warmth, value),
            StatType::WeaponSpeed => self.weapon_speed = Stat::new(StatType::WeaponSpeed, value),
            StatType::WoodcuttingPower => {
                self.woodcutting_power = Stat::new(StatType::WoodcuttingPower, value)
            }
        }
    }

    /// # Add
    /// Adds the given stats together.
    pub fn add(stats: impl IntoIterator<Item = Stats>) -> Self {
        let mut total_stats = Self::empty();
        for stat in stats {
            total_stats.armour.value += stat.armour.value;
            total_stats.attack_power.value += stat.attack_power.value;
            total_stats.block_points.value += stat.block_points.value;
            total_stats.mining_power.value += stat.mining_power.value;
            total_stats.precision.value += stat.precision.value;
            total_stats.ranged_accuracy.value += stat.ranged_accuracy.value;
            total_stats.ranged_damage.value += stat.ranged_damage.value;
            total_stats.speed.value += stat.speed.value;
            total_stats.warmth.value += stat.warmth.value;
            total_stats.weapon_speed.value += stat.weapon_speed.value;
            total_stats.woodcutting_power.value += stat.woodcutting_power.value;
        }
        total_stats
    }
}
