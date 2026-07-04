use friday_llm::{LlmProvider, LlmRequest, ChatMessage};
use friday_memory::MemoryStore;
use friday_plugins::Plugin;
use std::sync::Arc;

pub struct AgentOrchestrator {
    llm: Arc<dyn LlmProvider>,
    memory: Arc<MemoryStore>,
    plugins: Vec<Arc<dyn Plugin>>,
}

impl AgentOrchestrator {
    pub fn new(llm: Arc<dyn LlmProvider>, memory: Arc<MemoryStore>) -> Self {
        Self {
            llm,
            memory,
            plugins: Vec::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Arc<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    /// Ask the orchestrator to solve a task, critiquing it first (Ruthless Mode).
    pub async fn run_task(&self, user_input: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Analyze and critique the task (Ruthless Mode)
        let system_prompt = "You are the Lead Architect of Friday AI.
Analyze the user's task. You must critique it objectively.
Detail:
- Technical feasibility
- Risks and complexity
- Suggest the best implementation strategy.
Do NOT blindly agree. Be objective.";

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_input.to_string(),
            },
        ];

        let req = LlmRequest {
            messages,
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };

        let response = self.llm.generate(req).await?;
        
        // Log critique to memory
        let conversation_id = uuid::Uuid::new_v4().to_string();
        let _ = self.memory.save_message(&format!("{}-critique", conversation_id), "assistant", &response.content);

        Ok(response.content)
    }
}

pub struct AutomationAgent {
    pub desktop: friday_desktop::DesktopOperator,
    pub browser: friday_browser::BrowserOperator,
}

impl AutomationAgent {
    pub fn new() -> Self {
        Self {
            desktop: friday_desktop::DesktopOperator::new(),
            browser: friday_browser::BrowserOperator::new(),
        }
    }

    pub fn run_workflow(&mut self, action: &str, target: &str) -> Result<String, Box<dyn std::error::Error>> {
        match action {
            "browser_open" => {
                let title = self.browser.navigate_and_get_title(target)?;
                Ok(format!("Successfully opened browser at {}. Title: {}", target, title))
            }
            "desktop_screenshot" => {
                self.desktop.capture_screen(target)?;
                Ok(format!("Saved screenshot to {}", target))
            }
            _ => Err("Unknown action pattern for AutomationAgent".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_agent_workflows() {
        let mut agent = AutomationAgent::new();
        
        // Test browser mock action
        let res = agent.run_workflow("browser_open", "https://google.com").unwrap();
        assert!(res.contains("Title: Mock Title for https://google.com"));

        // Test desktop mock screenshot action
        let temp_dir = tempfile::tempdir().unwrap();
        let path_str = temp_dir.path().join("shot.png").to_string_lossy().to_string();
        let res_shot = agent.run_workflow("desktop_screenshot", &path_str).unwrap();
        assert!(res_shot.contains("Saved screenshot"));
    }
}
