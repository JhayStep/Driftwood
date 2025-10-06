use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Actor id for replicas
pub type Actor = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VClock(pub BTreeMap<Actor, u64>);

impl VClock {
    pub fn inc(&mut self, actor: &Actor) -> u64 {
        let e = self.0.entry(actor.clone()).or_default();
        *e += 1;
        *e
    }
    pub fn merge(&mut self, other: &VClock) {
        for (a, c) in &other.0 {
            let e = self.0.entry(a.clone()).or_default();
            *e = (*e).max(*c);
        }
    }
    pub fn dominates(&self, other: &VClock) -> bool {
        // self >= other elementwise
        for (a, c) in &other.0 {
            if self.0.get(a).unwrap_or(&0) < c { return false; }
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Dot { pub actor: Actor, pub counter: u64 }
