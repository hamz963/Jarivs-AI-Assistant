use crate::{LlmProvider, LlmRequest, LlmResponse};
use serde_json::json;

pub struct OpenAiProvider {
    pub api_key: String,
    pub model: String,
    pub api_base: String,
    client: reqwest::Client,
}

impl OpenAiProvider {
    pub fn new(api_key: String, model: String, api_base: Option<String>) -> Self {
        Self {
            api_key,
            model,
            api_base: api_base.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenAiProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/chat/completions", self.api_base);
        
        let body = json!({
            "model": self.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens,
        });

        let res = self.client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(format!("OpenAI API error: {}", err_text).into());
        }

        let json_val: serde_json::Value = res.json().await?;
        let content = json_val["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Failed to parse response content from OpenAI response")?
            .to_string();

        Ok(LlmResponse { content })
    }
}
