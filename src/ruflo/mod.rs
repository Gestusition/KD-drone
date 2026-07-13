//! Ruflo AI-agent capabilities integration.
//!
//! Integrates the claude-flow daemon's AgentDB, AIDefence, and SONA intelligence
//! hooks into the ruview-swarm orchestrator via a trait-based backend.
//!
//! Feature gate: `ruflo`. The `RufloBackend` trait and `MockRufloBackend` are always
//! compiled so tests can use them without enabling the `ruflo` feature. Only
//! `HttpRufloBackend` (which requires `reqwest` + `serde_json`) is gated.

pub mod backend;
pub mod mission_summary;
pub mod mock_backend;

#[cfg(feature = "ruflo")]
pub mod http_backend;

pub use backend::{MavlinkScanResult, MissionMemoryEntry, PatternEntry, RufloBackend, RufloError};
pub use mission_summary::MissionSummary;
pub use mock_backend::MockRufloBackend;

#[cfg(feature = "ruflo")]
pub use http_backend::HttpRufloBackend;
