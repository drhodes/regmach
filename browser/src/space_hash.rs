use crate::types::*;
use regmach::dsp::types as rdt;
use std::collections::{HashMap, HashSet};

impl SpaceHash {
    pub fn new() -> SpaceHash {
        SpaceHash { store: HashMap::new() }
    }

    pub fn insert(ent: u8) {
        // let id = ent.id();
        // let bbox = ent.bbox();
    }
}
