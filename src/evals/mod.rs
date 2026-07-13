//! ADR-171 statistically-rigorous evaluation harness (Stage 1, kinematic).
//!
//! Produces SAR + MARL metrics over a seeded N-seed × M-episode matrix with
//! IQM + 95% stratified-bootstrap CIs, a (sigma, kappa) CSI-noise sweep, and
//! GDOP-stratified localization error. Generates evals/RESULTS.md.
//!
//! Stage 2 (Gazebo/PX4 SITL high-fidelity, false-alarm + collision rate on the
//! median seeds) is a follow-on — see ADR-171 §6.1.
pub mod gdop;
pub mod metrics;
pub mod report;
pub mod runner;
pub mod stats;

pub use gdop::gdop;
pub use metrics::{AggregateMetrics, EpisodeMetrics};
pub use report::render_results_md;
pub use runner::{run_matrix, EvalConfig, NoiseLevel};
pub use stats::{iqm, stratified_bootstrap_ci, ConfidenceInterval};
