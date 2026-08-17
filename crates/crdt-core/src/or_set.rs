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
    pub adds: HashMap<T, HashSet<Dot>>,
    pub removes: HashSet<Dot>,
}

impl<T: Eq + Hash + Clone> Default for OrSet<T> {
    fn default() -> Self {
        Self {
            actor: "anon".into(),
            clock: VClock::default(),
            adds: HashMap::default(),
            removes: HashSet::default(),
        }
    }
}

impl<T: Eq + Hash + Clone> OrSet<T> {
    pub fn new(actor: impl Into<Actor>) -> Self {
        Self {
            actor: actor.into(),
            ..Default::default()
        }
    }

    pub fn add(&mut self, t: T) {
        let c = self.clock.inc(&self.actor);
        let dot = Dot {
            actor: self.actor.clone(),
            counter: c,
        };

        self.adds.entry(t).or_default().insert(dot);
    }

    pub fn remove(&mut self, t: &T) {
        if let Some(dots) = self.adds.get(t) {
            for d in dots {
                self.removes.insert(d.clone());
            }
        }
    }

    pub fn contains(&self, t: &T) -> bool {
        if let Some(dots) = self.adds.get(t) {
            dots.iter().any(|d| !self.removes.contains(d))
        } else {
            false
        }
    }

    pub fn elements(&self) -> Vec<T> {
        self.adds
            .iter()
            .filter_map(|(t, dots)| {
                if dots.iter().any(|d| !self.removes.contains(d)) {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn merge(&mut self, other: &Self) {
        self.clock.merge(&other.clock);

        for (t, odots) in &other.adds {
            self.adds
                .entry(t.clone())
                .or_default()
                .extend(odots.iter().cloned());
        }

        self.removes.extend(other.removes.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn added_element_is_present() {
        let mut set = OrSet::new("node-a");
        let item = "apple".to_string();

        set.add(item.clone());

        assert!(set.contains(&item));
    }

    #[test]
    fn removed_element_is_not_present() {
        let mut set = OrSet::new("node-a");
        let item = "apple".to_string();

        set.add(item.clone());
        set.remove(&item);

        assert!(!set.contains(&item));
    }

    #[test]
    fn elements_returns_only_active_values() {
        let mut set = OrSet::new("node-a");

        let apple = "apple".to_string();
        let banana = "banana".to_string();

        set.add(apple.clone());
        set.add(banana.clone());
        set.remove(&apple);

        let elements = set.elements();

        assert!(!elements.contains(&apple));
        assert!(elements.contains(&banana));
    }

    #[test]
    fn merge_combines_elements_from_two_replicas() {
        let mut left = OrSet::new("node-a");
        let mut right = OrSet::new("node-b");

        let apple = "apple".to_string();
        let banana = "banana".to_string();

        left.add(apple.clone());
        right.add(banana.clone());

        left.merge(&right);

        assert!(left.contains(&apple));
        assert!(left.contains(&banana));
    }

    #[test]
    fn removal_propagates_after_merge() {
        let mut left = OrSet::new("node-a");
        let mut right = OrSet::new("node-b");

        let item = "apple".to_string();

        left.add(item.clone());

        right.merge(&left);
        right.remove(&item);

        left.merge(&right);

        assert!(!left.contains(&item));
    }

    #[test]
    fn concurrent_add_survives_observed_remove() {
        let mut left = OrSet::new("node-a");
        let mut right = OrSet::new("node-b");

        let item = "apple".to_string();

        left.add(item.clone());
        right.add(item.clone());

        left.remove(&item);

        left.merge(&right);

        assert!(left.contains(&item));
    }
}
