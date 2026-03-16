pub struct Config {
    pub port: u16,
    pub ai_grpc_url: String,
    pub db_service_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080),
            ai_grpc_url: std::env::var("AI_GRPC_URL").unwrap_or_else(|_| "http://relio-ai:50051".into()),
            db_service_url: std::env::var("DB_SERVICE_URL").unwrap_or_else(|_| "http://relio-back-db:8082".into()),
        }
    }
}
