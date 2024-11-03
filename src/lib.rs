use std::io;
use reqwest::{Client, Method};
use safety_settings::SafetySettings;
use thiserror::Error;

pub mod safety_settings;

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
///## Example usage:
/// ```rs
///let mut convo = Conversation::new(
///    std::env::var("GEMINI_API_KEY").unwrap(), // Replace with however you want to get your API key
///    "gemini-1.5-flash".to_string() // Use a model from get_models() 
///);
///
///let response = convo.prompt("Hello World!")await.unwrap();
///println!("{0:?}", a.text);
/// ```
#[derive(Debug)]
pub struct Conversation {
    token: String,
    model: String,
    history: Vec<Response>,
    safety_settings: SafetySettings
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
        Self { token, model, history: vec!(), safety_settings: safety_settings::default() }
    }

    /// Update the safety settings to different thresholds from [safety_settings]
    /// ## Example:
    /// ```rs 
    /// let mut convo = Conversation::new(
    ///     "ABC123".to_string,
    ///     "gemini-1.5-flash".to_string
    /// ).update_safety_settings(safety_settings::default());
    /// ```
    pub fn update_safety_settings(&mut self, settings: SafetySettings) {
        self.safety_settings = settings;
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

        let mut data = json::object! {
            "safetySettings": [],
            "contents": []
        };
        for i in self.history.iter() {
            data["contents"].push(json::object! {
                "parts": [{"text": i.text.clone()}],
                "role": i.role.clone()
            })?
        };
        for i in self.safety_settings.iter() {
            data["safetySettings"].push(json::object! {
                "category": i.0,
                "threshold": i.1.get_actual()
            })?
        };

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

/// Get available models
/// ## Important
/// Most of these models **don't** currently work with this crate, so proceed
/// with caution if you want to use different models.
///
/// The safe options that have been tested so far are:
/// - `gemini-1.5-flash`
/// - `gemini-1.5-pro`
/// - `gemini-1.0-pro`
pub async fn get_models(token: &str) -> Result<Vec<String>, GeminiError> {
    let mut models: Vec<String> = vec![];
    let request = reqwest::get(format!(
        "https://generativelanguage.googleapis.com/v1beta/models?key={0}",
        token
    )).await?.text().await?;
    let response_json = json::parse(&request)?;
    for i in response_json["models"].members() {
        models.push(i["name"].to_string().strip_prefix("models/").unwrap().to_string());
    }

    Ok(models) 
}

