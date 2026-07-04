use crate::{LlmProvider, LlmRequest, LlmResponse};
use serde_json::json;

pub struct OllamaProvider {
    pub model: String,
    pub api_base: String,
    client: reqwest::Client,
}

impl OllamaProvider {
    pub fn new(model: String, api_base: Option<String>) -> Self {
        Self {
            model,
            api_base: api_base.unwrap_or_else(|| "http://localhost:11434/api".to_string()),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OllamaProvider {
    async fn generate(&self, request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/generate", self.api_base);

        let system_prompt = request.messages.iter()
            .filter(|m| m.role == "system")
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let user_prompt = request.messages.iter()
            .filter(|m| m.role == "user")
            .map(|m| m.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let body = json!({
            "model": self.model,
            "prompt": user_prompt,
            "system": system_prompt,
            "stream": false,
        });

        let res = self.client.post(&url)
            .json(&body)
            .send()
            .await?;

        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(format!("Ollama API error: {}", err_text).into());
        }

        let json_val: serde_json::Value = res.json().await?;
        let content = json_val["response"]
            .as_str()
            .ok_or("Failed to parse response field from Ollama")?
            .to_string();

        Ok(LlmResponse { content })
    }
}
