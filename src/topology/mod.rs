//! Swarm topology: Raft consensus, gossip dissemination, mesh management.
//!
//! Cooperative coordination primitives — leader election + log replication for
//! consistent shared state across a civilian fleet. General-purpose distributed
//! consensus, not military swarming. See `NOTICE`.

pub mod gossip;
pub mod mesh;
pub mod raft;

pub use gossip::GossipState;
pub use mesh::MeshTopology;
pub use raft::{RaftConfig, RaftNode, RaftRole};
