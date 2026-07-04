use friday_llm::{LlmProvider, LlmRequest, ChatMessage};
use std::sync::Arc;

pub struct WhisperFlowRefiner {
    llm: Arc<dyn LlmProvider>,
}

impl WhisperFlowRefiner {
    pub fn new(llm: Arc<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    /// Refine the user input using the LLM (Whisper Flow style).
    pub async fn refine_prompt(&self, raw_input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let system_prompt = "You are the Whisper Flow Prompt Refinement Engine.
Your task is to take a raw transcript or voice input and output a refined, clean, structured instruction.
Remove filler words, correct typos, clarify user intent, and optimize the prompt.
Output ONLY the clean refined prompt.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: raw_input.to_string(),
            },
        ];

        let req = LlmRequest {
            messages,
            temperature: Some(0.1),
            max_tokens: Some(300),
        };

        let res = self.llm.generate(req).await?;
        Ok(res.content.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use friday_llm::LlmResponse;
    use async_trait::async_trait;

    struct DummyLlm;

    #[async_trait]
    impl LlmProvider for DummyLlm {
        async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
            Ok(LlmResponse {
                content: "Refined output: Create a web server".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_refiner_flow() {
        let llm = Arc::new(DummyLlm);
        let refiner = WhisperFlowRefiner::new(llm);
        let output = refiner.refine_prompt("uh... please build... like a web server, you know?").await.unwrap();
        assert_eq!(output, "Refined output: Create a web server");
    }
}
