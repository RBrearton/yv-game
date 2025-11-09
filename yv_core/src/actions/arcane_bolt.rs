use crate::prelude::*;

/// # Arcane bolt
/// An arcane bolt is a magical projectile that can be cast by a character.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArcaneBolt {
    pub action_core: actions::ActionCore,
    pub target: Identity,
}

impl HasDisplayName for ArcaneBolt {
    fn display_name(&self) -> &str {
        well_known_terms::actions::ARCANE_BOLT
    }
}

impl Describable for ArcaneBolt {
    fn description(&self) -> &str {
        well_known_terms::descriptions::actions::ARCANE_BOLT
    }
}

impl Identifiable for ArcaneBolt {
    fn identity(&self) -> Identity {
        self.action_core.identity()
    }
}

impl HasRequirements for ArcaneBolt {
    fn requirements(&self) -> &Requirements {
        &well_known_terms::requirements::ARCANE_BOLT
    }
}

impl ActionLike for ArcaneBolt {
    fn range(&self) -> f32 {
        well_known_terms::distances::LONG_DISTANCE_ACTION
    }

    fn cooldown(&self) -> Duration {
        well_known_terms::cooldowns::ARCANE_BOLT
    }

    fn on_completion(&self, _world: &World) -> Commands {
        Command::DealDamage {
            target: self.target,
            damage: 10,
        }
        .into()
    }

    fn cast_time(&self) -> Option<Duration> {
        Some(well_known_terms::cast_times::ARCANE_BOLT)
    }

    fn target(&self) -> Option<&Identity> {
        Some(&self.target)
    }

    fn performer(&self) -> &Identity {
        &self.action_core.performer
    }
}
