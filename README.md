# Friday AI - Autonomous System Automation & Engineering Co-worker

Friday AI is an advanced, high-performance local AI assistant and autonomous engineering co-worker built from scratch in **Rust** for maximum memory safety, concurrency, and speed. It integrates real-time voice streams, file/ZIP processors, Whisper Flow prompt refinement, and sandboxed developer co-worker capabilities.

---

## 🚀 Comparison: Friday AI vs. OpenJarvis vs. OpenClaw

| Feature | **Friday AI (This Project)** | **OpenJarvis** | **OpenClaw** |
|:---|:---|:---|:---|
| **Core Language** | **Rust (Native)** | Python | Python |
| **System Drivers**| Direct OS Key/Mouse & Headless Browser | App Shell / Python SDK | Chat Application Hooks |
| **Safety Sandbox**| Integrated Terminal & Command Filters | External Containers | User Approvals |
| **Prompt Pipeline**| Whisper Flow Real-time Refiner | Standard LLM Templates | Vector Database RAG |
| **Real-time Audio**| Native CPAL/Rodio Stream Capture | External API Calls | App Voice Notes |
| **Diagnostics** | Auto-debugging Rust Compiler loop | Standard trace prints | External LSP plugins |

---

## 🛠️ System Architecture & Workspace Modules

Friday AI is organized as a modular Cargo workspace:

*   **`friday-core`**: Configuration manager (wake word, LLM model choice) and hardware profiles detector.
*   **`friday-llm`**: Multi-provider adapters (OpenAI, Gemini, Ollama, Anthropic) and `html2md` URL crawler/scraper.
*   **`friday-memory`**: High-performance SQLite agent message history and telemetry database.
*   **`friday-refiner`**: Whisper Flow speech filler refiner that transforms raw voice requests into target actions.
*   **`friday-generator`**: Direct vector SVG layout writers and directory boilerplate creators.
*   **`friday-terminal`**: Command line execution sandbox that sanitizes commands to prevent destructive shell operations.
*   **`friday-git`**: Repository manager that tracks branch logs and commit history.
*   **`friday-diagnostics`**: Automates compiler error resolution, running diagnostic checks to debug files.
*   **`friday-api`**: High-performance Axum REST API web server.

---

## 📦 Getting Started

### Prerequisites
- Rust Toolchain (Stable 2021 edition)
- MinGW-W64 GCC (Windows GNU target)

### Install dependencies and build:
```bash
cargo build --release
```

### Run the pipeline checks directly:
```bash
cargo run --bin friday
```

### Run all tests:
```bash
cargo test
```

---

## 📄 License
This project is open-source under the MIT License.
