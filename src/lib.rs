//!HI CHAT

use reqwest::{Client, Error, Method};

pub struct Conversation<'a> {
    pub token: &'a str,
    pub model: &'a str,
}

impl<'a> Conversation<'a> {
    /// Used to contact the Gemini API
    pub async fn prompt(self, input: &str) -> Result<String, Error> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{0}:generateContent?key={1}",
            self.model, self.token
        );
        let data = "{
        \"contents\": [{
        \"parts\": [{\"text\": \""
            .to_owned()
            + input
            + "\"}]
        }] 
    }";
        println!("{data}");

        let client = Client::new();
        let request = client
            .request(Method::POST, url)
            .header("Content-Type", "application/json")
            .body(data)
            .build();
        let response = client.execute(request?).await?;
        let response_text = response.text().await?;
        Ok(response_text)
    }
}
