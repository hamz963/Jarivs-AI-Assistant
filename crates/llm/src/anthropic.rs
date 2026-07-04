use crate::{LlmProvider, LlmRequest, LlmResponse};

pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for AnthropicProvider {
    async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        // Placeholder implementation for Anthropic
        Ok(LlmResponse {
            content: "Anthropic placeholder response".to_string(),
        })
    }
}
