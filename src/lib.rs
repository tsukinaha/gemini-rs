use std::{
    fs::File, io::{self, Write}
};
use json::JsonValue;
use reqwest::{Client, Method};
use thiserror::Error;

/// Error type for the Gemini API
#[derive(Error, Debug)]
pub enum GeminiError {
    /// Error type for HTTP request errors
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    /// Error type for IO errors
    #[error("IO operation failed: {0}")]
    IoError(#[from] io::Error),
    
    /// Error type for JSON parsing errors (you shouldn't get this one unless something bad happened)
    #[error("JSON parsing failed: {0}")]
    JsonError(#[from] json::Error),
    
    /// Error type for parsing
    #[error("Response parsing failed: {0}")]
    ParseError(String),
}

/// Represents a conversation with Gemini
/// Example usage:
/// ```rs
/// // main.rs
/// use gemini_rs::Conversation;

/// #[tokio::main]
/// async fn main() {
///    let mut convo = Conversation::new(
///        std::env::var("GEMINI_API_KEY").unwrap(), // Replace with however you want to get your API key
///        "gemini-1.5-flash".to_string() // Replace with the desired model from https://ai.google.dev/gemini-api/docs/models/gemini
///    );
///
///    let a = convo.prompt("If you had to describe Risk of Rain 2 in one word, what word would it be?").await.unwrap();
///    println!("{0:?}", a.text);
///    let b = convo.prompt("Now explain your reasoning").await.unwrap();
///   println!("{0:?}", b.text);
///}
///```
#[derive(Debug)]
pub struct Conversation {
    pub token: String,
    pub model: String,
    history: Vec<Response>
}

/// Holds a response from Gemini
#[derive(Debug)]
pub struct Response {
    pub text: String,
    role: String
}

impl Conversation {
    /// Creates a new conversation instance
    pub fn new(token: String, model: String) -> Self {
        Self { token, model, history: vec!() }
    }

    /// Sends a prompt to the Gemini API and returns the response
    pub async fn prompt(&mut self, input: &str) -> Result<Response, GeminiError> {
        self.history.push(
            Response{ text: input.to_string(), role: "user".to_string() }
        );

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{0}:generateContent?key={1}",
            self.model, self.token
        );

        //let data = format!(
        //    r#"{{
        //        "contents": [{{
        //            "parts": [{{
        //                "text": "{}"
        //            }}]
        //        }}]
        //    }}"#,
        //    input.replace("\"", "\\\"")
        //);

        let mut data = json::object! {
            "contents": []
        };
        for i in self.history.iter() {
            data["contents"].push(json::object! {
                "parts": [{"text": i.text.clone()}],
                "role": i.role.clone()
            })?
        }

        let client = Client::new();
        let request = client
            .request(Method::POST, url)
            .header("Content-Type", "application/json")
            .body(data.dump())
            .build()?;

        let http_response = client.execute(request).await?;
        let response_json = http_response.text().await?;
        let response_dict = json::parse(&response_json)?;

        let response_text = response_dict["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| GeminiError::ParseError("Failed to extract response text".to_string()))?;

        self.history.push(
            Response { text: response_text.to_string(), role: "model".to_string() }
        );

        Ok(Response {
            text: response_text.to_string(),
            role: "model".to_string()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conversation() {
        let mut conv = Conversation::new(
            "your-api-key".to_string(),
            "gemini-pro".to_string(),
        );
        
        let result = conv.prompt("Hello, how are you?").await;
        assert!(result.is_ok());
    }
}
