# Driftwood — Offline‑First CRDT Sync Engine (Rust)

Driftwood is a senior-level Rust project that demonstrates advanced distributed systems engineering. It is an offline-first synchronization engine powered by CRDTs (Conflict-Free Replicated Data Types) — allowing multiple peers to modify shared data concurrently and still reach consistent state without central coordination.

The system uses delta-state merging, vector clocks, and gossip-based synchronization to ensure conflict-free replication across distributed nodes. It’s designed as a foundation for collaborative and resilient offline applications like Notion, Figma, or Obsidian Sync.

Core Components

crdt-core → Core data structures implementing LWW-Map, OR-Set, and Vector Clock CRDTs.

sync-node → A Tokio-based service for peer-to-peer delta synchronization, persistence (via sled), and message exchange.

demo-cli → Command-line utility to create and edit local documents, demonstrating real-time merging and sync between nodes.

Conflict-Free Replication: Automatic resolution of concurrent edits using CRDT semantics.

💾 Local Persistence: Fast, embedded storage using sled for durability and offline operation.

🔄 Delta Synchronization: Efficient state propagation through compact deltas instead of full data dumps.

🌐 Peer Gossip Protocol: Eventually consistent state achieved through lightweight, periodic message exchange.

🧩 Composable Library: crdt-core can be reused independently for other distributed or offline-first systems.

🧱 Written in Rust: Safe concurrency, low-level performance, and type safety for production-grade reliability.

echnology Stack

Language: Rust (Edition 2021)

Async Runtime: Tokio

Serialization: Serde + Bincode

Persistence: Sled

Logging & Tracing: Tracing + tracing-subscriber

CLI Framework: ClapDriftwood/
├── Cargo.toml
├── crates/
│   ├── crdt-core/      # Core CRDT logic (LWW-Map, OR-Set, Vector Clock)
│   ├── sync-node/      # Networking, gossip, and persistence node
│   └── demo-cli/       # CLI demo for creating and syncing notes
└── README.md
Roadmap

 Implement gossip anti-entropy loop with digests

 Add RGA (Replicated Growable Array) for collaborative text editing

 Introduce secure QUIC transport with TLS

 Add TUI editor that syncs notes offline

 Integrate OpenTelemetry metrics and property-based testing

 Publish crdt-core as a standalone crate on crates.ioExample Use Cases

Offline-first document editing

Peer-to-peer databases

Real-time collaboration apps

Distributed configuration systems

Edge or IoT data synchronizationJhaydn Steplight
Third-Year Computer Science Major, Minor in Mathematics
Saint Martin’s University — Lacey, WashingtonThis project is licensed under the MIT License.
You are free to use, modify, and distribute it for educational or commercial purposes.
