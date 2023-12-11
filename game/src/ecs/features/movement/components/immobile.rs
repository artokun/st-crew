use bevy::prelude::*;

use crate::ecs::features::movement::models::ImmobileReason;

#[derive(Component)]
pub struct Immobile {
    reasons: ImmobileReason,
}

impl Immobile {
    pub const fn new(reasons: ImmobileReason) -> Self {
        Self { reasons }
    }

    /// Returns `true` if the component should be removed from the entity.
    pub fn should_remove(&self) -> bool {
        self.reasons.is_empty()
    }

    /// Checks if the entity is immobile for the given reason.
    pub fn has(&self, reason: ImmobileReason) -> bool {
        self.reasons.contains(reason)
    }

    /// Adds a reason for immobility.
    ///
    /// # Panics
    ///
    /// Adding a reason that is already present is considered a bug
    /// and will panic in debug builds.
    pub fn add(&mut self, reason: ImmobileReason) {
        debug_assert!(
            !self.reasons.contains(reason),
            "Reason {:?} is already present",
            reason
        );

        self.reasons.insert(reason);
    }

    /// Removes a reason for immobility.
    ///
    /// # Panics
    ///
    /// Removing a reason that is not present is considered a bug
    /// and will panic in debug builds.
    pub fn remove(&mut self, commands: &mut Commands, entity: Entity, reason: ImmobileReason) {
        debug_assert!(
            self.reasons.contains(reason),
            "Reason {:?} is not present",
            reason
        );

        self.reasons.remove(reason);

        if self.reasons.is_empty() {
            commands.entity(entity).remove::<Immobile>();
        }
    }
}
