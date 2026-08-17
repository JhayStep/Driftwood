# Driftwood — Offline-First CRDT Sync Engine

Driftwood is a Rust-based distributed systems project exploring offline-first data synchronization using **Conflict-Free Replicated Data Types (CRDTs)**.

The project focuses on allowing independent replicas to modify data while disconnected and later merge their state deterministically without requiring a central coordinator.

Driftwood is organized as a Rust workspace separating the core CRDT implementation, synchronization infrastructure, and command-line demonstration tools.

## Core Components

### `crdt-core`

Reusable CRDT data structures and causal-tracking logic.

Currently includes:

- **Vector Clock (VClock)** for tracking causal progress between replicas
- **Observed-Remove Set (OR-Set)** with dot-based tombstone tracking
- **Last-Write-Wins Map (LWW-Map)** with timestamp-based conflict resolution
- Deterministic tie-breaking using actor IDs and counters
- Merge operations for reconciling replica state

The core CRDT behavior is covered by automated Rust tests.

### `sync-node`

Synchronization-node infrastructure built with Rust and Tokio.

This component is intended to provide the networking and persistence layer connecting CRDT replicas.

### `demo-cli`

Command-line interface for experimenting with Driftwood and demonstrating the synchronization architecture.

## CRDT Behavior

Driftwood's core data structures are designed around eventual convergence between replicas.

### Vector Clocks

Each replica maintains logical counters associated with actor IDs. Vector clocks allow Driftwood to track causal progress and compare replica state.

Implemented behavior includes:

- Actor counter increments
- Clock merging
- Dominance comparisons
- Detection of concurrent/independent replica histories

### Observed-Remove Set

The OR-Set tracks additions using unique dots consisting of an actor ID and counter.

Implemented behavior includes:

- Adding elements
- Removing observed elements
- Merging sets from independent replicas
- Propagating removals between replicas
- Preserving concurrent additions that were not observed by a remove operation

### Last-Write-Wins Map

The LWW-Map associates values with timestamps and causal metadata.

Implemented behavior includes:

- Key/value assignment
- Timestamp-based conflict resolution
- Deterministic actor/counter tie-breaking
- Replica merging
- Preservation of independent keys

## Automated Testing

The CRDT core includes automated tests covering vector clocks, OR-Set behavior, and LWW-Map conflict resolution.

Run the complete test suite with:

```bash
cargo test --workspace
```

The project also uses automated continuous integration to verify changes pushed to the repository.

CI checks include:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Technology Stack

- **Language:** Rust (Edition 2021)
- **Async Runtime:** Tokio
- **Serialization:** Serde + Bincode
- **Persistence:** sled
- **Logging:** tracing + tracing-subscriber
- **CLI:** Clap
- **Testing:** Rust built-in test framework
- **CI:** GitHub Actions

## Project Structure

```text
Driftwood/
├── Cargo.toml
├── crates/
│   ├── crdt-core/
│   │   └── Core CRDT implementations
│   ├── sync-node/
│   │   └── Synchronization and persistence infrastructure
│   └── demo-cli/
│       └── Command-line demonstration application
└── README.md
```

## Getting Started

### Prerequisites

Install the Rust toolchain using Rustup.

Verify the installation:

```bash
rustc --version
cargo --version
```

### Clone Driftwood

```bash
git clone https://github.com/JhayStep/Driftwood.git
cd Driftwood
```

### Build

Build the complete workspace:

```bash
cargo build --workspace
```

### Test

Run all automated tests:

```bash
cargo test --workspace
```

### Code Quality

Check formatting:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

## Engineering Goals

Driftwood is being developed as an exploration of several distributed-systems problems:

- Offline-first application architecture
- Eventual consistency
- Conflict-free replication
- Causal tracking
- Deterministic conflict resolution
- Peer-to-peer synchronization
- Durable local state
- Testing distributed data structures

## Roadmap

Planned improvements include:

- [ ] Complete and validate the gossip anti-entropy synchronization loop
- [ ] Add end-to-end multi-node synchronization tests
- [ ] Add property-based tests for CRDT convergence
- [ ] Add RGA (Replicated Growable Array) support for collaborative text editing
- [ ] Explore secure QUIC/TLS transport
- [ ] Build a terminal-based collaborative editor
- [ ] Add observability and performance metrics
- [ ] Publish `crdt-core` as a standalone Rust crate

## Potential Use Cases

The concepts explored in Driftwood can support systems such as:

- Offline-first applications
- Collaborative software
- Peer-to-peer data stores
- Distributed configuration systems
- Edge and IoT synchronization

## Author

**Jhaydn Steplight**  
Computer Science Major, Mathematics Minor  
Saint Martin's University — Lacey, Washington  
Expected Graduation: May 2027

## License

This project is licensed under the MIT License.