use std::marker::PhantomData;

use bevy::{prelude::*, utils::Uuid};

use crate::utils::short_type_name;

pub trait UniqueIdType: Send + Sync + 'static {}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueId<T: UniqueIdType> {
    phantom: PhantomData<T>,

    inner: Uuid,
}

impl<T> UniqueId<T>
where
    T: UniqueIdType,
{
    pub(crate) fn new_random() -> UniqueId<T> {
        UniqueId {
            phantom: PhantomData,

            inner: Uuid::new_v4(),
        }
    }

    pub fn into_inner(self) -> Uuid {
        self.inner
    }
}

impl<T> AsRef<Uuid> for UniqueId<T>
where
    T: UniqueIdType,
{
    fn as_ref(&self) -> &Uuid {
        &self.inner
    }
}

impl<T> std::fmt::Debug for UniqueId<T>
where
    T: UniqueIdType + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(short_type_name::<T>())
            .field(&self.inner)
            .finish()
    }
}

impl<T> std::fmt::Display for UniqueId<T>
where
    T: UniqueIdType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner.to_string())
    }
}
