pub mod asset;
pub mod composition;
pub mod io;
pub mod migration;
pub mod schema;
pub mod sequence;
pub mod settings;
pub mod validation;
pub mod xml_parser;

pub use asset::{AssetFingerprint, AssetKind, AssetRef};
pub use composition::{Composition, KeyframeChannel, Layer, LayerKind, Transform};
pub use io::{AutosaveManager, ProjectIo, RecoveryInfo};
pub use migration::MigrationRunner;
pub use schema::{NovaProject, PROJECT_SCHEMA_VERSION};
pub use sequence::{Clip, ClipItem, Sequence, Track, TrackKind};
pub use settings::ProjectSettings;
