use crate::types::*;
use rdt::EntityId;
use regmach::dsp::types as rdt;
use std::collections::hash_map::Values;
use std::collections::{HashMap, HashSet};

impl SpaceHash {
    pub fn new() -> SpaceHash {
        SpaceHash { store: HashMap::new(), space: HashMap::new() }
    }

    pub fn insert(&mut self, ent: Box<rdt::Entity>) -> EntityId {
        // let bbox = ent.bbox();
        EntityId(42)
    }

    pub fn entities_iter(&self) -> Values<'_, EntityId, Box<dyn rdt::Entity>> {
        self.store.values()
    }

    pub fn draw(&self, dsp: Box<dyn rdt::Display>) {}
}
