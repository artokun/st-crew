use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Deref)]
pub struct UniqueId(pub Uuid);

impl UniqueId {
    pub(crate) fn new_random() -> UniqueId {
        UniqueId(Uuid::new_v4())
    }
}
