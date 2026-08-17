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
        for (a, c) in &other.0 {
            if self.0.get(a).unwrap_or(&0) < c {
                return false;
            }
        }
        true
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Ord,
    PartialOrd,
)]
pub struct Dot {
    pub actor: Actor,
    pub counter: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_increases_actor_counter() {
        let mut clock = VClock::default();
        let actor = "node-a".to_string();

        let first = clock.inc(&actor);
        let second = clock.inc(&actor);

        assert_eq!(first, 1);
        assert_eq!(second, 2);
        assert_eq!(clock.0.get(&actor), Some(&2));
    }

    #[test]
    fn merge_takes_max_counter_for_each_actor() {
        let mut left = VClock::default();
        let mut right = VClock::default();

        let actor_a = "node-a".to_string();
        let actor_b = "node-b".to_string();

        left.inc(&actor_a);
        left.inc(&actor_a);

        right.inc(&actor_a);
        right.inc(&actor_b);
        right.inc(&actor_b);
        right.inc(&actor_b);

        left.merge(&right);

        assert_eq!(left.0.get(&actor_a), Some(&2));
        assert_eq!(left.0.get(&actor_b), Some(&3));
    }

    #[test]
    fn clock_dominates_older_clock() {
        let mut newer = VClock::default();
        let mut older = VClock::default();

        let actor = "node-a".to_string();

        older.inc(&actor);

        newer.inc(&actor);
        newer.inc(&actor);

        assert!(newer.dominates(&older));
        assert!(!older.dominates(&newer));
    }

    #[test]
    fn equal_clocks_dominate_each_other() {
        let mut first = VClock::default();
        let mut second = VClock::default();

        let actor = "node-a".to_string();

        first.inc(&actor);
        second.inc(&actor);

        assert!(first.dominates(&second));
        assert!(second.dominates(&first));
    }

    #[test]
    fn independent_actors_do_not_dominate_each_other() {
        let mut first = VClock::default();
        let mut second = VClock::default();

        let actor_a = "node-a".to_string();
        let actor_b = "node-b".to_string();

        first.inc(&actor_a);
        second.inc(&actor_b);

        assert!(!first.dominates(&second));
        assert!(!second.dominates(&first));
    }
}