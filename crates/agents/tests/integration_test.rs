use friday_core::AppConfig;
use friday_memory::MemoryStore;
use friday_llm::{LlmProvider, LlmRequest, LlmResponse, ChatMessage};
use friday_agents::{AgentOrchestrator, AutomationAgent};
use std::sync::Arc;
use async_trait::async_trait;

struct MockLlm;

#[async_trait]
impl LlmProvider for MockLlm {
    async fn generate(&self, _request: LlmRequest) -> Result<LlmResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(LlmResponse {
            content: "Mock critique: The desktop screenshot workflow is feasible.".to_string(),
        })
    }
}

#[tokio::test]
async fn test_full_friday_integration_workflow() {
    // 1. Load config (Phase 1)
    let config = AppConfig::default();
    assert_eq!(config.voice.wake_word, "friday");

    // 2. Initialize database memory (Phase 1)
    let memory = Arc::new(MemoryStore::new_in_memory().unwrap());
    memory.set_preference("system_status", "initialized").unwrap();

    // 3. Setup orchestrator (Phase 1)
    let llm = Arc::new(MockLlm);
    let orchestrator = AgentOrchestrator::new(llm.clone(), memory.clone());

    // 4. Critique user task using LLM and Orchestrator (Phase 1 - Ruthless Mode)
    let critique = orchestrator.run_task("Save a screenshot of my desktop").await.unwrap();
    assert!(critique.contains("feasible"));

    // 5. Run automation workflow via Desktop Operator (Phase 2)
    let mut automation = AutomationAgent::new();
    let temp_dir = tempfile::tempdir().unwrap();
    let shot_path = temp_dir.path().join("desktop_shot.png");
    let shot_path_str = shot_path.to_string_lossy().to_string();

    let flow_res = automation.run_workflow("desktop_screenshot", &shot_path_str).unwrap();
    assert!(flow_res.contains("Saved screenshot"));
    assert!(shot_path.exists());

    // 6. Log final state to database memory (Phase 1)
    memory.save_message("session-1", "system", &flow_res).unwrap();
    
    // Assert preferences are persistent
    let status = memory.get_preference("system_status").unwrap();
    assert_eq!(status, Some("initialized".to_string()));

    // 7. Whisper Flow Prompt Refinement (Phase 3)
    let refiner = friday_refiner::WhisperFlowRefiner::new(llm.clone());
    let refined_prompt = refiner.refine_prompt("umm Friday, please... uh... make a design poster...").await.unwrap();
    assert_eq!(refined_prompt, "Mock critique: The desktop screenshot workflow is feasible."); // Mocked by MockLlm

    // 8. ZIP Processing & Folder Traversal (Phase 3)
    let archive_path = temp_dir.path().join("archive.zip");
    let extract_path = temp_dir.path().join("extracted_project");
    
    let file = std::fs::File::create(&archive_path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("source.txt", zip::write::SimpleFileOptions::default()).unwrap();
    use std::io::Write;
    zip.write_all(b"Hello world from integration!").unwrap();
    zip.finish().unwrap();

    friday_files::FileProcessor::extract_zip(&archive_path, &extract_path).unwrap();
    let scanned_files = friday_files::FileProcessor::scan_directory(&extract_path).unwrap();
    assert_eq!(scanned_files.len(), 1);
    assert_eq!(std::fs::read_to_string(&scanned_files[0]).unwrap(), "Hello world from integration!");

    // 9. SVG Poster Asset & Code Generator (Phase 3)
    let svg_poster_path = temp_dir.path().join("out.svg");
    friday_generator::ProjectGenerator::generate_svg_poster("Friday AI", "Digital Partner", &svg_poster_path).unwrap();
    assert!(svg_poster_path.exists());
    let svg_raw = std::fs::read_to_string(&svg_poster_path).unwrap();
    assert!(svg_raw.contains("Digital Partner"));

    // 10. Web Scraper & Link API processor (Phase 3)
    let _crawler = friday_llm::UrlCrawler::new();
    // We use a mock HTML parsing check directly to avoid external network calls during automated test runs
    let parsed_md = html2md::parse_html("<div><h1>Hello Links</h1></div>");
    assert!(parsed_md.contains("Hello Links"));
}
