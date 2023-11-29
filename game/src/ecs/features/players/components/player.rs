use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Deref)]
pub struct Player(Uuid);

impl Player {
    pub(crate) fn new_random() -> Player {
        Player(Uuid::new_v4())
    }
}
