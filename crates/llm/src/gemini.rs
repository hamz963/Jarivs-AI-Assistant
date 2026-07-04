use crate::{LlmProvider, LlmRequest, LlmResponse};

pub struct GeminiProvider {
    pub api_key: String,
    pub model: String,
    client: reqwest::Client,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for GeminiProvider {
    async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Placeholder implementation for Gemini
        Ok(LlmResponse {
            content: "Gemini placeholder response".to_string(),
        })
    }
}
