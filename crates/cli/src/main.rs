use friday_core::{AppConfig, detect_hardware};
use friday_memory::MemoryStore;
use friday_llm::{LlmProvider, LlmRequest, LlmResponse};
use friday_refiner::WhisperFlowRefiner;
use friday_generator::ProjectGenerator;
use friday_agents::AutomationAgent;
use friday_terminal::TerminalSandbox;
use friday_git::GitController;
use friday_diagnostics::DiagnosticsRunner;
use friday_api::ApiServer;
use std::sync::Arc;
use async_trait::async_trait;

struct MockLlm;

#[async_trait]
impl LlmProvider for MockLlm {
    async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(LlmResponse {
            content: "Refined prompt: generate_svg_poster \"Friday AI Operating System\" \"Memory Safe & Fast\"".to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Booting Friday AI Operating System ===");

    // 1. Detect hardware & settings
    let hw = detect_hardware();
    println!("Hardware Detected: Platform={}, CPU={}, RAM={:.1}GB", hw.platform, hw.cpu_brand, hw.ram_gb);

    let config = AppConfig::default();
    println!("Loaded Settings: wake_word='{}', model='{}'", config.voice.wake_word, config.llm.model);

    // 2. Initialize storage
    let temp_dir = tempfile::tempdir()?;
    let db_path = temp_dir.path().join("friday_state.db");
    let memory = Arc::new(MemoryStore::new(&db_path)?);
    memory.set_preference("system_status", "running")?;
    println!("State database initialized at: {:?}", db_path);

    // 3. Prompt Refiner Pipeline (Whisper Flow)
    let raw_input = "uh, Friday... please make... uh... a poster logo thing for Friday AI with text Digital Partner";
    println!("Raw Speech Input: \"{}\"", raw_input);

    let llm = Arc::new(MockLlm);
    let refiner = WhisperFlowRefiner::new(llm);
    let refined = refiner.refine_prompt(raw_input).await.map_err(|e| e as Box<dyn std::error::Error>)?;
    println!("Whisper Flow Refined Action: \"{}\"", refined);

    // 4. Generator Output Execution
    let poster_path = temp_dir.path().join("friday_poster.svg");
    ProjectGenerator::generate_svg_poster("Friday AI", "Digital Partner", &poster_path)?;
    println!("SVG Poster Generated successfully at: {:?}", poster_path);

    // 5. Automation Workflow Execution
    let mut automation = AutomationAgent::new();
    let web_res = automation.run_workflow("browser_open", "https://friday.ai")?;
    println!("Orchestrated Action Result: {}", web_res);

    // 6. Memory store archival
    memory.save_message("session-1", "user", raw_input)?;
    memory.save_message("session-1", "assistant", &refined)?;
    memory.save_message("session-1", "system", &web_res)?;
    println!("Session traces archived to database.");

    // 7. Developer Coworker Operations (Phase 4)
    println!("=== Testing Developer Coworker (Phase 4) Services ===");
    
    // a. Sandboxed Terminal test
    let sandbox_out = TerminalSandbox::execute_command("echo Hello Sandbox!")?;
    println!("Sandbox output: {}", sandbox_out);

    // b. Git tracking test
    let git_status = GitController::get_repository_status(".")?;
    println!("Git working tree status (abbreviated):\n{}", git_status.lines().next().unwrap_or(""));

    // c. Diagnostics compiler test
    let build_diag = DiagnosticsRunner::compile_and_get_diagnostics("echo build success")?;
    println!("Compiler diagnostics run:\n{}", build_diag);

    // d. Axum REST service setup verify
    let router = ApiServer::build_router();
    println!("Axum API Service Router built successfully.");

    println!("=== Pipeline E2E Workflow Completed Successfully ===");
    Ok(())
}
