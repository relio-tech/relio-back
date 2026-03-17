use std::sync::Arc;
use axum::{extract::ws::WebSocketUpgrade, extract::State, response::IntoResponse, routing::get, Router};
use dashmap::DashMap;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod ws;
mod grpc;

#[derive(Clone)]
pub struct AppState {
    pub sessions: Arc<DashMap<String, broadcast::Sender<String>>>,
    pub ai_client: Arc<tokio::sync::RwLock<Option<grpc::AiClient>>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("relio_back=debug".parse().unwrap()))
        .init();
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();

    let state = AppState {
        sessions: Arc::new(DashMap::new()),
        ai_client: Arc::new(tokio::sync::RwLock::new(None)),
    };

    let ai_addr = config.ai_grpc_url.clone();
    tokio::spawn({
        let state = state.clone();
        async move {
            loop {
                match grpc::AiClient::connect(&ai_addr).await {
                    Ok(client) => {
                        tracing::info!("Connected to AI service at {}", ai_addr);
                        *state.ai_client.write().await = Some(client);
                        break;
                    }
                    Err(e) => {
                        tracing::warn!("AI service not ready: {}. Retrying in 3s...", e);
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    }
                }
            }
        }
    });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/health", get(|| async { "ok" }))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("relio-back listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws::handler::handle_connection(socket, state))
}
