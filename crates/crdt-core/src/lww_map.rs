use crate::vclock::{Actor, Dot, VClock};
use fxhash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Timestamped assignment (LWW-Register per key) with causal context (dot)
/// for delta-state CRDT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stamp {
    pub dot: Dot,
    pub ts_micros: i128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwMap<K, V>
where
    K: Eq + Hash,
{
    pub actor: Actor,
    pub clock: VClock,
    pub assigns: HashMap<K, (Stamp, V)>,
}

impl<K, V> Default for LwwMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        Self {
            actor: "anon".into(),
            clock: VClock::default(),
            assigns: HashMap::default(),
        }
    }
}

impl<K, V> LwwMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(actor: impl Into<Actor>) -> Self {
        Self {
            actor: actor.into(),
            ..Default::default()
        }
    }

    pub fn put(&mut self, key: K, val: V, now_micros: i128) {
        let c = self.clock.inc(&self.actor);

        let dot = Dot {
            actor: self.actor.clone(),
            counter: c,
        };

        let stamp = Stamp {
            dot,
            ts_micros: now_micros,
        };

        let entry = self
            .assigns
            .entry(key)
            .or_insert_with(|| (stamp.clone(), val.clone()));

        // LWW: pick larger timestamp; break ties by (actor, counter).
        if stamp.ts_micros > entry.0.ts_micros
            || (stamp.ts_micros == entry.0.ts_micros
                && (stamp.dot.actor.clone(), stamp.dot.counter)
                    > (entry.0.dot.actor.clone(), entry.0.dot.counter))
        {
            *entry = (stamp, val);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.assigns.get(key).map(|(_, v)| v)
    }

    pub fn merge(&mut self, other: &Self) {
        self.clock.merge(&other.clock);

        for (k, (s, v)) in &other.assigns {
            let e = self
                .assigns
                .entry(k.clone())
                .or_insert_with(|| (s.clone(), v.clone()));

            if s.ts_micros > e.0.ts_micros
                || (s.ts_micros == e.0.ts_micros
                    && (s.dot.actor.clone(), s.dot.counter)
                        > (e.0.dot.actor.clone(), e.0.dot.counter))
            {
                *e = (s.clone(), v.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_and_get_value() {
        let mut map = LwwMap::new("node-a");
        let key = "name".to_string();

        map.put(key.clone(), "Jhaydn".to_string(), 100);

        assert_eq!(map.get(&key), Some(&"Jhaydn".to_string()));
    }

    #[test]
    fn newer_timestamp_replaces_older_value() {
        let mut map = LwwMap::new("node-a");
        let key = "status".to_string();

        map.put(key.clone(), "old".to_string(), 100);
        map.put(key.clone(), "new".to_string(), 200);

        assert_eq!(map.get(&key), Some(&"new".to_string()));
    }

    #[test]
    fn older_timestamp_does_not_replace_newer_value() {
        let mut map = LwwMap::new("node-a");
        let key = "status".to_string();

        map.put(key.clone(), "new".to_string(), 200);
        map.put(key.clone(), "old".to_string(), 100);

        assert_eq!(map.get(&key), Some(&"new".to_string()));
    }

    #[test]
    fn merge_uses_newer_timestamp() {
        let mut left = LwwMap::new("node-a");
        let mut right = LwwMap::new("node-b");

        let key = "status".to_string();

        left.put(key.clone(), "left".to_string(), 100);
        right.put(key.clone(), "right".to_string(), 200);

        left.merge(&right);

        assert_eq!(left.get(&key), Some(&"right".to_string()));
    }

    #[test]
    fn equal_timestamp_uses_actor_as_tiebreaker() {
        let mut left = LwwMap::new("node-a");
        let mut right = LwwMap::new("node-b");

        let key = "status".to_string();

        left.put(key.clone(), "from-a".to_string(), 100);
        right.put(key.clone(), "from-b".to_string(), 100);

        left.merge(&right);

        assert_eq!(left.get(&key), Some(&"from-b".to_string()));
    }

    #[test]
    fn merge_preserves_unique_keys_from_both_replicas() {
        let mut left = LwwMap::new("node-a");
        let mut right = LwwMap::new("node-b");

        let key_a = "first".to_string();
        let key_b = "second".to_string();

        left.put(key_a.clone(), "value-a".to_string(), 100);
        right.put(key_b.clone(), "value-b".to_string(), 100);

        left.merge(&right);

        assert_eq!(left.get(&key_a), Some(&"value-a".to_string()));
        assert_eq!(left.get(&key_b), Some(&"value-b".to_string()));
    }
}
