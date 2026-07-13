pub mod actor;
pub mod learning;
pub mod observation;
pub mod reward;
pub mod role_attention;
pub mod trainer;
pub mod training_loop;

pub use actor::{ActorAction, ActorConfig, MappoActor};
pub use learning::{shaped_reward, CuriosityModule, LearningPattern, MetaAdapter};
pub use observation::LocalObservation;
pub use reward::{RewardCalculator, RewardContext};
pub use role_attention::{triangulation_geometry_penalty, NodeRole, RoleAttention};
pub use trainer::{DomainRandomizationConfig, TrainingConfig, TrainingMode};
pub use training_loop::{ppo_update, PpoConfig, ReplayBuffer, Transition, UpdateStats};

#[cfg(feature = "train")]
pub mod candle_ppo;
#[cfg(feature = "train")]
pub use candle_ppo::{select_device, CandleActorCritic, CandlePpoConfig, CandleTrainer};
