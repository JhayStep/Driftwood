use crate::vclock::{Actor, Dot, VClock};
use fxhash::FxHashMap as HashMap;
use fxhash::FxHashSet as HashSet;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Observed-Remove Set with tombstones via dot tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrSet<T: Eq + Hash + Clone> {
    pub actor: Actor,
    pub clock: VClock,
    pub adds: HashMap<T, HashSet<Dot>>, // element -> dots
    pub removes: HashSet<Dot>,
}

impl<T: Eq + Hash + Clone> Default for OrSet<T> {
    fn default() -> Self { Self { actor: "anon".into(), clock: VClock::default(), adds: HashMap::default(), removes: HashSet::default() } }
}

impl<T: Eq + Hash + Clone> OrSet<T> {
    pub fn new(actor: impl Into<Actor>) -> Self { Self { actor: actor.into(), ..Default::default() } }

    pub fn add(&mut self, t: T) {
        let c = self.clock.inc(&self.actor);
        let dot = Dot { actor: self.actor.clone(), counter: c };
        self.adds.entry(t).or_default().insert(dot);
    }

    pub fn remove(&mut self, t: &T) {
        if let Some(dots) = self.adds.get(t) { for d in dots { self.removes.insert(d.clone()); } }
    }

    pub fn contains(&self, t: &T) -> bool {
        if let Some(dots) = self.adds.get(t) { dots.iter().any(|d| !self.removes.contains(d)) } else { false }
    }

    pub fn elements(&self) -> Vec<T> {
        self.adds.iter().filter_map(|(t, dots)| if dots.iter().any(|d| !self.removes.contains(d)) { Some(t.clone()) } else { None }).collect()
    }

    pub fn merge(&mut self, other: &Self) {
        self.clock.merge(&other.clock);
        for (t, odots) in &other.adds { self.adds.entry(t.clone()).or_default().extend(odots.iter().cloned()); }
        self.removes.extend(other.removes.iter().cloned());
    }
}
