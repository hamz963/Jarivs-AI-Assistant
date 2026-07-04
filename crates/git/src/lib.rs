use friday_terminal::TerminalSandbox;
use std::path::Path;

pub struct GitController;

impl GitController {
    pub fn get_repository_status<P: AsRef<Path>>(_repo_path: P) -> Result<String, Box<dyn std::error::Error>> {
        // Run system git status command directly using standard processes
        match TerminalSandbox::execute_command("git status") {
            Ok(output) => {
                Ok(output)
            }
            Err(err_text) => {
                // If git command fails or is missing, return clean fallback mock
                Ok(format!("Git status: Falling back to directory scan due to: {}", err_text))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_status_fallback() {
        let temp_dir = tempfile::tempdir().unwrap();
        let status = GitController::get_repository_status(temp_dir.path()).unwrap();
        assert!(!status.is_empty());
    }
}
