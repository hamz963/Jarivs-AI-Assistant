use serde::{Deserialize, Serialize};
use std::path::Path;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub model: String,
    pub api_base: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub database_url: String,
    pub use_vector: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub wake_word: String,
    pub tts_provider: String,
    pub stt_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub llm: LlmConfig,
    pub memory: MemoryConfig,
    pub voice: VoiceConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            llm: LlmConfig {
                provider: "openai".to_string(),
                api_key: None,
                model: "gpt-4o".to_string(),
                api_base: None,
            },
            memory: MemoryConfig {
                database_url: "sqlite://friday.db".to_string(),
                use_vector: false,
            },
            voice: VoiceConfig {
                wake_word: "friday".to_string(),
                tts_provider: "kokoro".to_string(),
                stt_provider: "whisper".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.llm.provider, "openai");
        assert_eq!(config.llm.model, "gpt-4o");
        assert_eq!(config.memory.use_vector, false);
        assert_eq!(config.voice.wake_word, "friday");
    }
}
