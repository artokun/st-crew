use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
    utils::tracing,
};

use crate::ecs::features::movement::{components::Immobile, models::ImmobileReason};

#[derive(SystemParam)]
pub struct ImmobilityMut<'w, 's> {
    commands: Commands<'w, 's>,
    query: Query<'w, 's, (Entity, Option<&'static mut Immobile>)>,
}

impl ImmobilityMut<'_, '_> {
    /// Adds an immobility reason to an entity, creating the `Immobile` component if necessary.
    pub fn add(&mut self, entity: Entity, reason: ImmobileReason) {
        match self.query.get_mut(entity) {
            Ok((_, Some(mut immobile))) if !immobile.has(reason) => {
                immobile.add(reason);
            }

            Ok((_, Some(_))) => {
                tracing::warn!("entity is already immobile for this reason");
            }

            Ok((_, None)) => {
                self.commands.entity(entity).insert(Immobile::new(reason));
            }

            Err(QueryEntityError::NoSuchEntity(_)) => {
                panic!("entity does not exist, did you try to add immobility to an entity that has not yet fully spawned?");
            }

            Err(err) => {
                tracing::error!("failed to add immobile reason to entity: {}", err);
            }
        }
    }

    /// Removes an immobility reason from an entity, removing the `Immobile` component if necessary.
    pub fn remove(&mut self, entity: Entity, reason: ImmobileReason) {
        match self.query.get_mut(entity) {
            Ok((_, Some(mut immobile))) if immobile.has(reason) => {
                immobile.remove(&mut self.commands, entity, reason);

                if immobile.should_remove() {
                    self.commands.entity(entity).remove::<Immobile>();
                }
            }

            Ok((_, Some(_))) => {
                tracing::warn!(
                    "cannot remove immobile reason, entity is immobile for other reasons"
                );
            }

            Ok((_, None)) => {
                tracing::warn!("cannot remove immobile reason, entity is not immobile");
            }

            Err(QueryEntityError::NoSuchEntity(_)) => {
                panic!("entity does not exist, did you try to remove immobility from a despawned entity?");
            }

            Err(err) => {
                tracing::error!("failed to remove immobile reason from entity: {}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use bevy::{
        app::{App, Update},
        ecs::{
            schedule::{apply_deferred, IntoSystemConfigs},
            system::Commands,
        },
    };

    use crate::ecs::features::movement::{
        components::Immobile, models::ImmobileReason, ImmobilityMut,
    };

    #[test]
    fn adds_component_if_not_present() {
        let mut app = App::new();

        let spawned_entity = Arc::new(Mutex::new(None));

        app.add_systems(
            Update,
            (
                {
                    let spawned_entity = Arc::clone(&spawned_entity);

                    move |mut commands: Commands| {
                        let entity = commands.spawn(()).id();

                        *spawned_entity.lock().unwrap() = Some(entity);
                    }
                },
                apply_deferred,
                {
                    let spawned_entity = Arc::clone(&spawned_entity);

                    move |mut immobility: ImmobilityMut| {
                        immobility.add(
                            spawned_entity
                                .lock()
                                .unwrap()
                                .expect("cannot add immobility, as spawn() was not called"),
                            ImmobileReason::PAUSED,
                        );
                    }
                },
            )
                .chain(),
        );

        app.update();

        let spawned_entity = spawned_entity
            .lock()
            .unwrap()
            .expect("entity was not spawned");

        let immobile = app
            .world
            .get::<Immobile>(spawned_entity)
            .expect("Immobile component not present");

        assert!(
            immobile.has(ImmobileReason::PAUSED),
            "entity should have the immobile reason"
        );
    }

    #[test]
    fn updates_component_if_present() {
        let mut app = App::new();

        let entity = app
            .world
            .spawn(())
            .insert(Immobile::new(ImmobileReason::PAUSED))
            .id();

        app.add_systems(Update, move |mut immobility: ImmobilityMut| {
            immobility.add(entity, ImmobileReason::NO_ENERGY);
        });

        app.update();

        let immobile = app
            .world
            .get::<Immobile>(entity)
            .expect("Immobile component not present");

        assert!(
            immobile.has(ImmobileReason::PAUSED),
            "entity should have the initial immobile reason"
        );

        assert!(
            immobile.has(ImmobileReason::NO_ENERGY),
            "entity should have the added immobile reason"
        );
    }

    #[test]
    fn removes_component_if_empty() {
        let mut app = App::new();

        let entity = app
            .world
            .spawn(())
            .insert(Immobile::new(ImmobileReason::PAUSED))
            .id();

        app.add_systems(Update, move |mut immobility: ImmobilityMut| {
            immobility.remove(entity, ImmobileReason::PAUSED);
        });

        app.update();

        assert!(
            app.world.get::<Immobile>(entity).is_none(),
            "Immobile component should have been removed"
        );
    }

    #[test]
    fn leaves_component_if_reasons_remain() {
        let mut app = App::new();

        let entity = app
            .world
            .spawn(())
            .insert(Immobile::new(
                ImmobileReason::PAUSED | ImmobileReason::NO_ENERGY,
            ))
            .id();

        app.add_systems(Update, move |mut immobility: ImmobilityMut| {
            immobility.remove(entity, ImmobileReason::PAUSED);
        });

        app.update();

        let immobile = app
            .world
            .get::<Immobile>(entity)
            .expect("Immobile component not present");

        assert!(
            !immobile.has(ImmobileReason::PAUSED),
            "entity should have removed an immobile reason"
        );

        assert!(
            immobile.has(ImmobileReason::NO_ENERGY),
            "entity should have the remaining immobile reason"
        );
    }
}
