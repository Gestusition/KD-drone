//! Drone swarm control system — ADR-148.
//!
//! Hierarchical-mesh topology · Raft consensus · MAPPO MARL · CSI sensing integration

pub mod allocation;
pub mod bench_support;
pub mod config;
pub mod demo;
pub mod evals;
pub mod failsafe;
pub mod formation;
pub mod integration;
pub mod marl;
pub mod orchestrator;
pub mod planning;
pub mod ruflo;
pub mod security;
pub mod sensing;
pub mod topology;
pub mod types;

pub use config::SwarmConfig;
pub use types::{
    ClusterId, CsiDetection, DroneState, FailSafeState, GridCell, NodeId, Position3D, SwarmError,
    SwarmResult, SwarmRole, SwarmTask, TaskId, TaskKind, Velocity3D,
};
