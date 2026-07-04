use std::fs;
use std::path::Path;

pub struct ProjectGenerator;

impl ProjectGenerator {
    pub fn generate_svg_poster<P: AsRef<Path>>(title: &str, subtitle: &str, save_path: P) -> Result<(), Box<dyn std::error::Error>> {
        let svg_content = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600" width="800" height="600">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#0f2027;stop-opacity:1" />
      <stop offset="50%" style="stop-color:#203a43;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#2c5364;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="800" height="600" fill="url(#grad)" />
  <text x="400" y="250" font-family="Outfit, Inter, sans-serif" font-size="48" fill="#ffffff" text-anchor="middle" font-weight="bold">{}</text>
  <text x="400" y="320" font-family="Inter, sans-serif" font-size="24" fill="#a0aec0" text-anchor="middle">{}</text>
  <circle cx="400" cy="450" r="40" fill="#4fd1c5" opacity="0.8" />
</svg>"##,
            title, subtitle
        );
        fs::write(save_path, svg_content)?;
        Ok(())
    }

    pub fn bootstrap_project_structure<P: AsRef<Path>>(root_dir: P, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = root_dir.as_ref();
        fs::create_dir_all(root.join("src"))?;
        fs::create_dir_all(root.join("tests"))?;
        fs::write(root.join("Cargo.toml"), format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"# , name
        ))?;
        fs::write(root.join("src/main.rs"), r#"fn main() {
    println!("Hello from Friday AI generated app!");
}
"#)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_poster_generation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("poster.svg");
        ProjectGenerator::generate_svg_poster("Friday AI", "Digital Partner", &path).unwrap();
        assert!(path.exists());
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("Friday AI"));
    }
}
