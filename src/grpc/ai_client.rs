use tonic::transport::Channel;

pub mod ai_proto {
    tonic::include_proto!("relio.ai");
}

pub struct AiClient {
    client: ai_proto::ai_service_client::AiServiceClient<Channel>,
}

impl AiClient {
    pub async fn connect(addr: &str) -> Result<Self, tonic::transport::Error> {
        let channel = Channel::from_shared(addr.to_string()).unwrap().connect().await?;
        Ok(Self { client: ai_proto::ai_service_client::AiServiceClient::new(channel) })
    }

    pub async fn start_session(&mut self, session_id: &str, mode: &str) -> Result<ai_proto::StartSessionResponse, tonic::Status> {
        let request = tonic::Request::new(ai_proto::StartSessionRequest {
            session_id: session_id.to_string(),
            mode: mode.to_string(),
            config: std::collections::HashMap::new(),
        });
        let response = self.client.start_live_session(request).await?;
        Ok(response.into_inner())
    }
}
