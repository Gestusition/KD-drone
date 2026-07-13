//! Demo scenario runner — synthetic CSI with configurable victim positions.
//!
//! Wires together a [`SyntheticCsiGenerator`] and pre-built [`DemoScenario`]
//! definitions for rapid scenario validation without real hardware.

pub mod scenario;
pub mod synthetic_csi;

pub use scenario::{DemoScenario, ScenarioResult};
pub use synthetic_csi::SyntheticCsiGenerator;
