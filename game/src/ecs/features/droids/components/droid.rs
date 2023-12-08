use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Deref)]
pub struct Droid(Uuid);

impl Droid {
    pub(crate) fn new_random() -> Droid {
        Droid(Uuid::new_v4())
    }
}
