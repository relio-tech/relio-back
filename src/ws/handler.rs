use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;
use super::messages::IncomingMessage;
use crate::AppState;

pub async fn handle_connection(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let session_id = Uuid::new_v4().to_string();
    tracing::info!("New WebSocket connection: session={}", session_id);

    let welcome = serde_json::json!({"type": "session_created", "session_id": session_id});
    let _ = sender.send(Message::Text(welcome.to_string().into())).await;

    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(text) => {
                let text_str: &str = &text;
                match serde_json::from_str::<IncomingMessage>(text_str) {
                    Ok(IncomingMessage::StartSession { config }) => {
                        tracing::info!("Starting session {} in mode: {}", session_id, config.mode);
                        if state.ai_client.read().await.is_some() {
                            let response = serde_json::json!({"type": "phase_change", "phase": "scanning", "session_id": session_id});
                            let _ = sender.send(Message::Text(response.to_string().into())).await;
                        }
                    }
                    Ok(IncomingMessage::VideoFrame { .. }) => {
                        if state.ai_client.read().await.is_some() {
                            let guidance = serde_json::json!({"type": "scan_guidance", "message": "Good angle. Slowly rotate the object clockwise.", "progress": 0.35});
                            let _ = sender.send(Message::Text(guidance.to_string().into())).await;
                        }
                    }
                    Ok(IncomingMessage::AudioChunk { .. }) => {}
                    Ok(IncomingMessage::StopSession) => {
                        tracing::info!("Stopping session {}", session_id);
                        let response = serde_json::json!({"type": "phase_change", "phase": "processing"});
                        let _ = sender.send(Message::Text(response.to_string().into())).await;
                    }
                    Ok(IncomingMessage::ExportRequest { export_type, .. }) => {
                        tracing::info!("Export request: {} for session {}", export_type, session_id);
                    }
                    Err(e) => tracing::warn!("Failed to parse message: {}", e),
                }
            }
            Message::Binary(data) => tracing::debug!("Received binary data: {} bytes", data.len()),
            Message::Close(_) => { tracing::info!("WebSocket closed: session={}", session_id); break; }
            _ => {}
        }
    }
}
