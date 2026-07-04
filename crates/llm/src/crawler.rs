pub struct UrlCrawler {
    client: reqwest::Client,
}

impl UrlCrawler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Fetches URL HTML content and parses it to clean Markdown
    pub async fn fetch_markdown(&self, url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let res = self.client.get(url)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(format!("Scraping failed with HTTP: {}", res.status()).into());
        }

        let html = res.text().await?;
        let md = html2md::parse_html(&html);
        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_parsing_to_markdown() {
        let raw_html = "<html><body><h1>Title</h1><p>Description text</p></body></html>";
        let md = html2md::parse_html(raw_html);
        assert!(md.contains("Title"));
        assert!(md.contains("Description text"));
    }
}
