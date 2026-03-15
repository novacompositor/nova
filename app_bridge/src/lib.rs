use engine::dispatcher::EngineDispatcher;
use engine_api::{EngineCommand, EngineEvent};
use std::sync::{mpsc, Mutex, OnceLock};

static DISPATCHER: OnceLock<EngineDispatcher> = OnceLock::new();
static EVENT_RECEIVER: OnceLock<Mutex<mpsc::Receiver<EngineEvent>>> = OnceLock::new();

#[cxx::bridge]
mod ffi {
    struct VideoFrameData {
        width: u32,
        height: u32,
        pts: i64,
        data: Vec<u8>,
    }

    extern "Rust" {
        fn init_engine() -> bool;
        fn execute_command(command_json: &str) -> String;
        fn get_video_frame(path: &str, time_ms: i64) -> VideoFrameData;
    }
}

pub fn init_engine() -> bool {
    let (tx, rx) = mpsc::channel();
    let dispatcher = EngineDispatcher::new(tx);
    // Initialize a blank project for the UI to play with immediately
    use engine_api::types::{AudioConfig, ColorProfile, FrameRate, RationalTime, Resolution};
    let _ = dispatcher.dispatch(EngineCommand::CreateProject {
        name: "Untitled Project".into(),
        resolution: Resolution {
            width: 1920,
            height: 1080,
        },
        frame_rate: FrameRate {
            num: 30000,
            den: 1001,
        },
        color_profile: ColorProfile::LinearSRGB,
        audio: AudioConfig {
            sample_rate: 48000,
            bit_depth: 24,
            channels: 2,
        },
    });
    // Add a composition to it
    let _ = dispatcher.dispatch(EngineCommand::AddComposition {
        name: "Main Comp".into(),
        resolution: Resolution {
            width: 1920,
            height: 1080,
        },
        frame_rate: FrameRate {
            num: 30000,
            den: 1001,
        },
        duration: RationalTime::new(10 * 30, 30),
        color_profile: ColorProfile::LinearSRGB,
    });

    let _ = DISPATCHER.set(dispatcher);
    let _ = EVENT_RECEIVER.set(Mutex::new(rx));

    println!("[app_bridge] Engine initialized from Rust.");
    true
}

pub fn execute_command(command_json: &str) -> String {
    println!("[app_bridge] Received command: {}", command_json);

    // Parse the JSON command loosely to intercept old FCPXML workflow
    match serde_json::from_str::<serde_json::Value>(command_json) {
        Ok(cmd_val) => {
            if let Some(cmd_type) = cmd_val.get("type").and_then(|t| t.as_str()) {
                if cmd_type == "OpenProject" {
                    if let Some(path) = cmd_val
                        .get("payload")
                        .and_then(|p| p.get("path"))
                        .and_then(|p| p.as_str())
                    {
                        if path.ends_with(".fcpxml") {
                            println!("[app_bridge] Triggering FCPXML Import for: {}", path);
                            if let Ok(xml_content) = std::fs::read_to_string(path) {
                                match project_schema::xml_parser::parse_fcpxml_sequence(
                                    &xml_content,
                                ) {
                                    Ok(sequence) => {
                                        let mut first_video_path = String::new();
                                        if let Some(track) = sequence.video_tracks.first() {
                                            if let Some(clip) = track.clips.first() {
                                                first_video_path = clip.name.clone();
                                            }
                                        }

                                        // Still returning the legacy format for EditRoom
                                        return serde_json::json!({
                                            "type": "ProjectChanged",
                                            "status": "success",
                                            "message": format!("Imported FCPXML with {} video tracks", sequence.video_tracks.len()),
                                            "sequence_id": sequence.id.to_string(),
                                            "first_video_path": first_video_path,
                                            "sequence": sequence
                                        }).to_string();
                                    }
                                    Err(e) => {
                                        return serde_json::json!({
                                            "status": "error",
                                            "message": format!("FCPXML Parse Error: {}", e)
                                        })
                                        .to_string();
                                    }
                                }
                            } else {
                                return serde_json::json!({
                                    "status": "error",
                                    "message": format!("Failed to read file: {}", path)
                                })
                                .to_string();
                            }
                        }
                    }
                } else if cmd_type == "SyncState" {
                    if let Some(dispatcher) = DISPATCHER.get() {
                        if let Ok(state) = dispatcher.state().lock() {
                            return serde_json::json!({
                                "status": "success",
                                "events": [
                                    {
                                        "type": "StateSynced",
                                        "payload": {
                                            "project": state.project.as_ref()
                                        }
                                    }
                                ]
                            })
                            .to_string();
                        }
                    }
                }
            }
        }
        Err(e) => {
            let err_json = serde_json::json!({
                "status": "error",
                "message": format!("Invalid JSON: {}", e)
            })
            .to_string();

            if command_json.contains("CreateSequence") {
                std::fs::write(
                    "/tmp/nova_create_seq_debug.log",
                    format!("CMD: {}\nERR: {}", command_json, err_json),
                )
                .ok();
            }

            return err_json;
        }
    }

    // Pass through EngineDispatcher
    if let Some(dispatcher) = DISPATCHER.get() {
        match serde_json::from_str::<EngineCommand>(command_json) {
            Ok(cmd) => {
                if let Err(e) = dispatcher.dispatch(cmd) {
                    return serde_json::json!({
                        "status": "error",
                        "message": format!("Engine error: {:?}", e)
                    })
                    .to_string();
                }

                // Drain events
                let mut events = Vec::new();
                if let Some(rx_lock) = EVENT_RECEIVER.get() {
                    if let Ok(rx) = rx_lock.lock() {
                        while let Ok(ev) = rx.try_recv() {
                            events.push(ev);
                        }
                    }
                }

                let rs_json = serde_json::json!({
                    "status": "success",
                    "events": events
                })
                .to_string();

                if command_json.contains("CreateSequence") {
                    std::fs::write(
                        "/tmp/nova_create_seq_debug.log",
                        format!("CMD: {}\nRES: {}", command_json, rs_json),
                    )
                    .ok();
                }

                return rs_json;
            }
            Err(e) => {
                return serde_json::json!({
                    "status": "error",
                    "message": format!("Failed to parse EngineCommand: {}", e)
                })
                .to_string();
            }
        }
    }

    serde_json::json!({
        "status": "error",
        "message": "Engine dispatcher not initialized"
    })
    .to_string()
}

pub fn get_video_frame(path: &str, _time_ms: i64) -> ffi::VideoFrameData {
    let path_obj = std::path::Path::new(path);

    // First, check if it's a static image (SVG, PSD, PNG, JPG, etc.)
    if media_image::probe_image(path_obj).is_ok() {
        match media_image::decode_image(path_obj) {
            Ok(frame) => {
                return ffi::VideoFrameData {
                    width: frame.width,
                    height: frame.height,
                    pts: 0,
                    data: frame.data,
                }
            }
            Err(e) => {
                eprintln!(
                    "[app_bridge] media_image decode error for {}: {:?}",
                    path, e
                );
                // Fallthrough to ffmpeg just in case
            }
        }
    }

    // Fallback: try ffmpeg decoding (for video/audio/other)
    match media_ffmpeg::decode::decode_first_frame(path_obj) {
        Ok(frame) => ffi::VideoFrameData {
            width: frame.width,
            height: frame.height,
            pts: frame.pts,
            data: frame.data,
        },
        Err(e) => {
            eprintln!("[app_bridge] FFmpeg decode error for {}: {:?}", path, e);
            ffi::VideoFrameData {
                width: 0,
                height: 0,
                pts: 0,
                data: Vec::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_execute_command_unsupported() {
        let result = execute_command(r#"{"type": "UnknownCommand"}"#);
        assert!(result.contains("processed (stub)"));
    }

    #[test]
    fn test_execute_command_fcpxml() {
        let mut tmp_file = NamedTempFile::new().unwrap();
        let path = tmp_file.path().with_extension("fcpxml");
        let mut file = fs::File::create(&path).unwrap();

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE fcpxml>
        <fcpxml version="1.9">
            <project name="Test Project">
                <sequence>
                    <spine>
                        <video name="Clip 1" offset="10s" start="2s" duration="5s" />
                    </spine>
                </sequence>
            </project>
        </fcpxml>"#;

        file.write_all(xml.as_bytes()).unwrap();

        let cmd = format!(
            r#"{{"type": "OpenProject", "payload": {{"path": "{}"}}}}"#,
            path.to_string_lossy()
        );
        let result = execute_command(&cmd);

        let response: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(response["status"], "success");
        assert!(response["message"]
            .as_str()
            .unwrap()
            .contains("1 video tracks"));

        fs::remove_file(path).unwrap();
    }
}
