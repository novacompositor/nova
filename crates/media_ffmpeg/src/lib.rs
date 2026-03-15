//! media_ffmpeg — FFmpeg integration for media ingest and export.
//! Uses the `ffmpeg-next` crate which wraps libavcodec/avformat/avutil.

pub mod decode;
pub mod encode;
pub mod error;
pub mod probe;

pub use decode::{AudioBuffer, VideoFrame};
pub use encode::{encode_gif, split_by_scenes, ConvertOptions};
pub use error::FfmpegError;
pub use probe::{MediaInfo, StreamInfo, StreamKind};
