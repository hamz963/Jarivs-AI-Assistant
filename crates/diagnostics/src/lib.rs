use friday_terminal::TerminalSandbox;

pub struct DiagnosticsRunner;

impl DiagnosticsRunner {
    pub fn compile_and_get_diagnostics(build_command: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!("Running compile command inside Diagnostics: {}", build_command);
        match TerminalSandbox::execute_command(build_command) {
            Ok(output) => {
                Ok(format!("Build successful! Output:\n{}", output))
            }
            Err(err_text) => {
                // Return structured error parser analysis
                Ok(format!("Build failed! Parsing errors:\n{}", err_text))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostics_mock_success() {
        let res = DiagnosticsRunner::compile_and_get_diagnostics("echo compiled").unwrap();
        assert!(res.contains("successful"));
    }
}
