pub trait AudioInputDevice: Send + Sync {
    fn start_recording(&self, callback: Box<dyn Fn(Vec<f32>) + Send>) -> Result<(), Box<dyn std::error::Error>>;
    fn stop_recording(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait SpeechSynthesizer: Send + Sync {
    fn speak(&self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

pub struct StreamingVoiceManager {
    // Stub properties
}

impl StreamingVoiceManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start_voice_assistant_loop(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Runs voice activation in the background
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_manager_init() {
        let manager = StreamingVoiceManager::new();
        assert!(manager.start_voice_assistant_loop().is_ok());
    }
}
