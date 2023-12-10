use std::ops::{Index, IndexMut};

use crate::{app::event::Event, components::Component};

pub struct Children<C> {
    children: Vec<C>,
}

impl<C> Default for Children<C> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl<C> Children<C>
where
    C: Component,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, child: C) {
        self.children.push(child);
    }

    pub fn extend(&mut self, children: impl IntoIterator<Item = C>) {
        self.children.extend(children);
    }

    pub fn remove_child(&mut self, index: usize) -> C {
        self.children.remove(index)
    }

    pub fn clear(&mut self) {
        self.children.clear();
    }

    /// Sends the event to each child in order until one of them handles it.
    ///
    /// Returns `true` if the event was handled.
    pub fn propagate_event(&mut self, event: &Event) -> bool {
        for child in &mut self.children {
            if child.handle_event(event) {
                return true;
            }
        }

        false
    }
}

impl<C> Index<usize> for Children<C> {
    type Output = C;

    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index]
    }
}

impl<C> IndexMut<usize> for Children<C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.children[index]
    }
}

impl<C> IntoIterator for Children<C> {
    type Item = C;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl<C> From<C> for Children<C> {
    fn from(child: C) -> Self {
        Self {
            children: vec![child],
        }
    }
}

impl<C> From<Vec<C>> for Children<C> {
    fn from(children: Vec<C>) -> Self {
        Self { children }
    }
}
