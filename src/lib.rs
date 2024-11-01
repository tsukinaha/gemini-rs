use std::{
    collections::HashMap, fs::File, io::{self, Write}
};
use json::JsonValue;
use reqwest::{Client, Method};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeminiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("IO operation failed: {0}")]
    IoError(#[from] io::Error),
    
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] json::Error),
    
    #[error("Response parsing failed: {0}")]
    ParseError(String),
}

#[derive(Hash, Eq, Debug, PartialEq)]
enum Role {
    User,
    Model
}

#[derive(Debug)]
pub struct Conversation {
    pub token: String,
    pub model: String,
    history: Vec<HashMap<Role, String>>
}

#[derive(Debug)]
pub struct Response {
    pub text: String,
}

impl Conversation {
    /// Creates a new conversation instance
    pub fn new(token: String, model: String) -> Self {
        Self { token, model, history: vec!() }
    }

    /// Sends a prompt to the Gemini API and returns the response
    pub async fn prompt(&mut self, input: &str) -> Result<Response, GeminiError> {
        let mut user_history = HashMap::new();
        user_history.insert(Role::User, input.to_string());
        self.history.push( user_history );

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{0}:generateContent?key={1}",
            self.model, self.token
        );

        let data = format!(
            r#"{{
                "contents": [{{
                    "parts": [{{
                        "text": "{}"
                    }}]
                }}]
            }}"#,
            input.replace("\"", "\\\"")
        );

        let client = Client::new();
        let request = client
            .request(Method::POST, url)
            .header("Content-Type", "application/json")
            .body(data)
            .build()?;

        let http_response = client.execute(request).await?;
        let response_json = http_response.text().await?;
        let response_dict = json::parse(&response_json)?;

        log_json(&response_dict)?;

        let response_text = response_dict["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| GeminiError::ParseError("Failed to extract response text".to_string()))?;

        let mut model_history = HashMap::new();
        model_history.insert(Role::Model, response_text.to_string());
        self.history.push( model_history );

        println!("{self:?}");

        Ok(Response {
            text: response_text.to_string(),
        })
    }
}

fn log_json(json: &JsonValue) -> Result<(), GeminiError> {
    let mut file = File::create("log.json")?;
    file.write_all(json.dump().as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conversation() {
        let conv = Conversation::new(
            "your-api-key".to_string(),
            "gemini-pro".to_string(),
        );
        
        let result = conv.prompt("Hello, how are you?").await;
        assert!(result.is_ok());
    }
}
