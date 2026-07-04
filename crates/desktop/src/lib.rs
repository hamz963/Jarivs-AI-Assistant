use enigo::{Enigo, Mouse, Keyboard, Settings};
use std::path::Path;

pub struct DesktopOperator {
    enigo: Enigo,
}

impl DesktopOperator {
    pub fn new() -> Self {
        // Initialize Enigo with default settings
        let enigo = Enigo::new(&Settings::default()).unwrap();
        Self { enigo }
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.move_mouse(x, y, enigo::Coordinate::Abs)?;
        Ok(())
    }

    pub fn click(&mut self, button: enigo::Button) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.button(button, enigo::Direction::Click)?;
        Ok(())
    }

    pub fn type_text(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.text(text)?;
        Ok(())
    }

    pub fn capture_screen<P: AsRef<Path>>(&self, save_path: P) -> Result<(), Box<dyn std::error::Error>> {
        // Capture screen fallback or mock screenshot generating a simple PNG file
        let img = image::ImageBuffer::from_fn(1920, 1080, |x, y| {
            if (x as i32 - 960).pow(2) + (y as i32 - 540).pow(2) < 40000 {
                image::Rgb([255_u8, 0_u8, 0_u8]) // red circle in center
            } else {
                image::Rgb([0_u8, 0_u8, 255_u8]) // blue background
            }
        });
        img.save(save_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_operator_init() {
        // Verify we can initialize and run mock captures
        let op = DesktopOperator::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("screenshot.png");
        op.capture_screen(&path).unwrap();
        assert!(path.exists());
    }
}
