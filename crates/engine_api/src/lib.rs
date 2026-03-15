/// ENGINE_API_VERSION — semver string for bridge compatibility checks.
pub const ENGINE_API_VERSION: &str = "0.1.0";

pub mod command;
pub mod error;
pub mod event;
pub mod query;
pub mod types;

pub use command::EngineCommand;
pub use error::EngineError;
pub use event::EngineEvent;
pub use query::{EngineQuery, QueryResult};
pub use types::{
    AssetId, AudioConfig, ColorProfile, CompositionId, LayerId, ProjectId, RationalTime, Resolution,
};
