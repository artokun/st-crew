use bevy::{prelude::*, utils::Uuid};

#[derive(Component, Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConnectionId(Uuid);

impl ConnectionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.hyphenated().fmt(f)
    }
}
