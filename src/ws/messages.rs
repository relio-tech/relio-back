use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    #[serde(rename = "start_session")]
    StartSession { config: SessionConfig },
    #[serde(rename = "video_frame")]
    VideoFrame { frame: String, timestamp: i64 },
    #[serde(rename = "audio_chunk")]
    AudioChunk { audio: String, timestamp: i64 },
    #[serde(rename = "stop_session")]
    StopSession,
    #[serde(rename = "export_request")]
    ExportRequest { export_type: String, config: serde_json::Value },
}

#[derive(Deserialize)]
pub struct SessionConfig {
    pub mode: String,
}

#[derive(Serialize)]
pub struct OutgoingMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}
