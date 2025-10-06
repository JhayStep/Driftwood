use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type DocId = String; // e.g., note id

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Msg {
    Hello { node_id: String },
    StateDigest { counts: BTreeMap<DocId, u64> }, // anti-entropy summary (per-doc version)
    Delta { doc: DocId, bytes: Vec<u8> },         // serialized delta (simple: full merge chunk for now)
    Pull { doc: DocId },
}
