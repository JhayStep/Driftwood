# Driftwood — Offline-First CRDT Sync Engine

Driftwood is a Rust-based distributed systems project exploring offline-first data synchronization using **Conflict-Free Replicated Data Types (CRDTs)**.

The project allows independent replicas to modify and persist data locally and synchronize their state between peers without requiring a central coordinator.

Driftwood is organized as a Rust workspace separating the core CRDT implementation, peer synchronization infrastructure, persistence layer, and command-line demonstration tools.

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

Peer synchronization and persistence infrastructure built with Rust and Tokio.

Currently supports:

- Independent synchronization nodes with unique actor IDs
- TCP-based peer communication
- Peer discovery through command-line peer configuration
- Anti-entropy synchronization between replicas
- State digest exchange
- Pull requests for missing or outdated documents
- Serialized delta/state transfer between nodes
- Persistent local storage using sled
- Recovery of synchronized data from local storage
- Multiple nodes listening on independent network addresses

Two-node synchronization has been manually verified by writing data to Node A, connecting Node B to Node A, synchronizing the document, stopping the nodes, and successfully reading the synchronized value from Node B's local database.

### `demo-cli`

Command-line interface for experimenting with Driftwood and demonstrating local CRDT operations.

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

## Peer Synchronization

Driftwood supports synchronization between independent nodes.

A node can maintain its own sled database and listen for peer connections. Another node can connect to it and use the anti-entropy protocol to synchronize document state.

The current synchronization flow includes:

1. Establishing a TCP connection between peers
2. Exchanging node information
3. Comparing available document state
4. Requesting required documents
5. Serializing document state with Bincode
6. Transferring the state between peers
7. Persisting synchronized data in the receiving node's sled database

This provides a working foundation for eventual peer-to-peer CRDT synchronization.

## Two-Node Demo

First, create data for Node A:

```bash
cargo run -p sync-node -- --node node-a --db ./data-node-a put test-sync title "Hello from Node A"
```

Start Node A:

```bash
cargo run -p sync-node -- --node node-a --listen 127.0.0.1:7070 --db ./data-node-a run
```

In another terminal, start Node B and connect it to Node A:

```bash
cargo run -p sync-node -- --node node-b --listen 127.0.0.1:7071 --db ./data-node-b run --peer 127.0.0.1:7070
```

After synchronization, stop Node B so its sled database lock is released.

Then verify Node B received the document:

```bash
cargo run -p sync-node -- --node node-b --db ./data-node-b get test-sync title
```

Expected output:

```text
Hello from Node A
```

This demonstrates persistent synchronization from one independent replica to another.

## Automated Testing

The CRDT core currently contains **17 automated tests** covering vector clocks, OR-Set behavior, and LWW-Map conflict resolution.

Run the complete test suite with:

```bash
cargo test
```

The project also uses GitHub Actions continuous integration to validate changes pushed to the repository.

CI verifies:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The CI pipeline currently passes formatting, Clippy, and automated testing checks.

## Technology Stack

- **Language:** Rust (Edition 2021)
- **Async Runtime:** Tokio
- **Networking:** TCP
- **Serialization:** Serde + Bincode
- **Persistence:** sled
- **Logging:** tracing + tracing-subscriber
- **CLI:** Clap
- **Testing:** Rust built-in test framework
- **CI:** GitHub Actions

## Project Structure

```text
Driftwood/
├── .github/
│   └── workflows/
│       └── rust.yml
├── Cargo.toml
├── Cargo.lock
├── crates/
│   ├── crdt-core/
│   │   └── Core CRDT implementations
│   ├── sync-node/
│   │   └── Peer synchronization and persistence infrastructure
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
cargo test
```

### Code Quality

Check formatting:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Engineering Goals

Driftwood is being developed as an exploration of several distributed-systems problems:

- Offline-first application architecture
- Eventual consistency
- Conflict-free replication
- Causal tracking
- Deterministic conflict resolution
- Peer-to-peer synchronization
- Anti-entropy protocols
- Durable local state
- Distributed data persistence
- Testing distributed data structures

## Current Progress

- [x] Vector Clock implementation
- [x] OR-Set implementation
- [x] LWW-Map implementation
- [x] CRDT merge and convergence tests
- [x] Local sled persistence
- [x] TCP peer communication
- [x] Two-node communication
- [x] State digest protocol
- [x] Pull-based synchronization
- [x] Persistent Node A → Node B synchronization
- [x] GitHub Actions CI
- [x] Strict Clippy validation
- [x] 17 automated CRDT tests

## Roadmap

Planned improvements include:

- [ ] Add automated end-to-end multi-node synchronization tests
- [ ] Expand anti-entropy synchronization for more complex replica scenarios
- [ ] Add bidirectional synchronization and conflict demonstrations
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