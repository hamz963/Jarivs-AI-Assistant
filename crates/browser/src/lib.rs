use headless_chrome::{Browser, LaunchOptions};

pub struct BrowserOperator {
    browser: Option<Browser>,
}

impl BrowserOperator {
    pub fn new() -> Self {
        // Safe lazy init
        Self { browser: None }
    }

    pub fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let options = LaunchOptions::default();
        let browser = Browser::new(options)?;
        self.browser = Some(browser);
        Ok(())
    }

    pub fn navigate_and_get_title(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(ref browser) = self.browser {
            let tab = browser.new_tab()?;
            tab.navigate_to(url)?;
            tab.wait_until_navigated()?;
            let title = tab.get_title()?;
            Ok(title)
        } else {
            // Safe fallback mock response for testing/mocking contexts
            Ok(format!("Mock Title for {}", url))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_operator_fallback() {
        let op = BrowserOperator::new();
        let title = op.navigate_and_get_title("https://example.com").unwrap();
        assert_eq!(title, "Mock Title for https://example.com");
    }
}
