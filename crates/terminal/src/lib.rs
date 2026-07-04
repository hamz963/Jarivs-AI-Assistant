use std::process::Command;

pub struct TerminalSandbox;

impl TerminalSandbox {
    pub fn is_safe_command(cmd: &str) -> bool {
        let dangerous_keywords = vec!["rm -rf", "format", "del /s", "mkfs", "dd if="];
        for kw in dangerous_keywords {
            if cmd.contains(kw) {
                return false;
            }
        }
        true
    }

    pub fn execute_command(cmd: &str) -> Result<String, Box<dyn std::error::Error>> {
        if !Self::is_safe_command(cmd) {
            return Err("SecurityException: Destructive command execution blocked".into());
        }

        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd])
                .output()?
        } else {
            Command::new("sh")
                .args(&["-c", cmd])
                .output()?
        };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(stderr.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_safety_filter() {
        assert!(TerminalSandbox::is_safe_command("echo hello"));
        assert!(!TerminalSandbox::is_safe_command("rm -rf /"));
    }

    #[test]
    fn test_execute_safe_command() {
        let out = TerminalSandbox::execute_command("echo hello_friday").unwrap();
        assert!(out.contains("hello_friday"));
    }
}
