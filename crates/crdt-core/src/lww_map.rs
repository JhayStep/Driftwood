use crate::vclock::{Actor, Dot, VClock};
use fxhash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// Timestamped assignment (LWW-Register per key) with causal context (dot) for delta-state CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stamp { pub dot: Dot, pub ts_micros: i128 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LwwMap<K, V>
where K: Eq + Hash + Clone, V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    pub actor: Actor,
    pub clock: VClock,
    pub assigns: HashMap<K, (Stamp, V)>,
}

impl<K, V> Default for LwwMap<K, V>
where K: Eq + Hash + Clone, V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self { Self { actor: "anon".into(), clock: VClock::default(), assigns: HashMap::default() } }
}

impl<K, V> LwwMap<K, V>
where K: Eq + Hash + Clone, V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(actor: impl Into<Actor>) -> Self { Self { actor: actor.into(), ..Default::default() } }

    pub fn put(&mut self, key: K, val: V, now_micros: i128) {
        let c = self.clock.inc(&self.actor);
        let dot = Dot { actor: self.actor.clone(), counter: c };
        let stamp = Stamp { dot, ts_micros: now_micros };
        let entry = self.assigns.entry(key).or_insert_with(|| (stamp.clone(), val.clone()));
        // LWW: pick larger timestamp; break ties by (actor, counter)
        if stamp.ts_micros > entry.0.ts_micros || (stamp.ts_micros == entry.0.ts_micros && (stamp.dot.actor, stamp.dot.counter) > (entry.0.dot.actor.clone(), entry.0.dot.counter)) {
            *entry = (stamp, val);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> { self.assigns.get(key).map(|(_, v)| v) }

    pub fn merge(&mut self, other: &Self) {
        self.clock.merge(&other.clock);
        for (k, (s, v)) in &other.assigns {
            let e = self.assigns.entry(k.clone()).or_insert_with(|| (s.clone(), v.clone()));
            if s.ts_micros > e.0.ts_micros || (s.ts_micros == e.0.ts_micros && (s.dot.actor.clone(), s.dot.counter) > (e.0.dot.actor.clone(), e.0.dot.counter)) {
                *e = (s.clone(), v.clone());
            }
        }
    }
}
