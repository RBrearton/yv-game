use crate::prelude::*;

/// # Stats
/// A struct that contains all possible stats.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    // Defensive.
    pub armour: Stat,
    pub block_points: Stat,
    pub vitality: Stat,

    // General.
    pub durability: Stat,
    pub speed: Stat,
    pub warmth: Stat,

    // Melee.
    pub attack_power: Stat,
    pub slashing_bonus: Stat,
    pub stabbing_bonus: Stat,
    pub crushing_bonus: Stat,

    // Melee + ranged.
    pub weapon_speed: Stat,

    // Ranged.
    pub precision: Stat,
    pub ranged_accuracy: Stat,
    pub ranged_damage: Stat,

    // Professions.
    pub mining_power: Stat,
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
    /// # Empty
    /// Explicitly creates a new set of stats with all values set to 0.
    pub const fn empty() -> Self {
        Self {
            armour: Stat::new(StatType::Armour, 0),
            block_points: Stat::new(StatType::BlockPoints, 0),
            speed: Stat::new(StatType::Speed, 0),
            warmth: Stat::new(StatType::Warmth, 0),
            durability: Stat::new(StatType::Durability, 0),
            attack_power: Stat::new(StatType::AttackPower, 0),
            slashing_bonus: Stat::new(StatType::SlashingBonus, 0),
            stabbing_bonus: Stat::new(StatType::StabbingBonus, 0),
            crushing_bonus: Stat::new(StatType::CrushingBonus, 0),
            mining_power: Stat::new(StatType::MiningPower, 0),
            precision: Stat::new(StatType::Precision, 0),
            ranged_accuracy: Stat::new(StatType::RangedAccuracy, 0),
            ranged_damage: Stat::new(StatType::RangedDamage, 0),
            weapon_speed: Stat::new(StatType::WeaponSpeed, 0),
            woodcutting_power: Stat::new(StatType::WoodcuttingPower, 0),
            vitality: Stat::new(StatType::Vitality, 0),
        }
    }

    /// # Get stat
    /// Gets the stat with the given type.
    pub fn get_stat(&self, stat_type: StatType) -> Stat {
        match stat_type {
            StatType::Armour => self.armour,
            StatType::Vitality => self.vitality,
            StatType::Durability => self.durability,
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
            StatType::SlashingBonus => self.slashing_bonus,
            StatType::StabbingBonus => self.stabbing_bonus,
            StatType::CrushingBonus => self.crushing_bonus,
        }
    }

    /// # Set stat
    /// Sets the stat with the given type to the given value.
    pub fn set_stat(&mut self, stat_type: StatType, value: i16) {
        match stat_type {
            StatType::Armour => self.armour = Stat::new(StatType::Armour, value),
            StatType::Vitality => self.vitality = Stat::new(StatType::Vitality, value),
            StatType::Durability => self.durability = Stat::new(StatType::Durability, value),
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
            StatType::SlashingBonus => {
                self.slashing_bonus = Stat::new(StatType::SlashingBonus, value)
            }
            StatType::StabbingBonus => {
                self.stabbing_bonus = Stat::new(StatType::StabbingBonus, value)
            }
            StatType::CrushingBonus => {
                self.crushing_bonus = Stat::new(StatType::CrushingBonus, value)
            }
        }
    }

    /// # Are all greater than or equal to
    /// Returns whether all the stats are greater than or equal to the given stats.
    pub fn are_all_greater_than_or_equal_to(&self, stats: &Stats) -> bool {
        let Stats {
            armour,
            vitality,
            block_points,
            durability,
            speed,
            warmth,
            attack_power,
            slashing_bonus,
            stabbing_bonus,
            crushing_bonus,
            weapon_speed,
            precision,
            ranged_accuracy,
            ranged_damage,
            mining_power,
            woodcutting_power,
        } = stats;

        self.armour.value >= armour.value
            && self.vitality.value >= vitality.value
            && self.block_points.value >= block_points.value
            && self.durability.value >= durability.value
            && self.speed.value >= speed.value
            && self.warmth.value >= warmth.value
            && self.attack_power.value >= attack_power.value
            && self.slashing_bonus.value >= slashing_bonus.value
            && self.stabbing_bonus.value >= stabbing_bonus.value
            && self.crushing_bonus.value >= crushing_bonus.value
            && self.weapon_speed.value >= weapon_speed.value
            && self.precision.value >= precision.value
            && self.ranged_accuracy.value >= ranged_accuracy.value
            && self.ranged_damage.value >= ranged_damage.value
            && self.mining_power.value >= mining_power.value
            && self.woodcutting_power.value >= woodcutting_power.value
    }
    /// # Add
    /// Adds the given stats together.
    pub fn add(stats: impl IntoIterator<Item = Stats>) -> Self {
        let mut total_stats = Self::empty();
        for stat in stats {
            // Destructure to force compile-time completeness checking.
            let Stats {
                armour,
                vitality,
                block_points,
                durability,
                speed,
                warmth,
                attack_power,
                slashing_bonus,
                stabbing_bonus,
                crushing_bonus,
                weapon_speed,
                precision,
                ranged_accuracy,
                ranged_damage,
                mining_power,
                woodcutting_power,
            } = stat;

            total_stats.armour.value += armour.value;
            total_stats.vitality.value += vitality.value;
            total_stats.block_points.value += block_points.value;
            total_stats.durability.value += durability.value;
            total_stats.speed.value += speed.value;
            total_stats.warmth.value += warmth.value;
            total_stats.attack_power.value += attack_power.value;
            total_stats.slashing_bonus.value += slashing_bonus.value;
            total_stats.stabbing_bonus.value += stabbing_bonus.value;
            total_stats.crushing_bonus.value += crushing_bonus.value;
            total_stats.weapon_speed.value += weapon_speed.value;
            total_stats.precision.value += precision.value;
            total_stats.ranged_accuracy.value += ranged_accuracy.value;
            total_stats.ranged_damage.value += ranged_damage.value;
            total_stats.mining_power.value += mining_power.value;
            total_stats.woodcutting_power.value += woodcutting_power.value;
        }
        total_stats
    }
}
