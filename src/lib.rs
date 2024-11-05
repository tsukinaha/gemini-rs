pub mod safety;
pub mod response;

use std::io;
use json::JsonValue;
use reqwest::{Client, Method};
use thiserror::Error;
use response::GeminiResponse;

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

    #[error("Invalid model: {0}")]
    ModelError(String)
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
    history: Vec<Message>,
    safety_settings: Vec<safety::SafetySetting>,
}

/// A part of a conversation, used to store history
#[derive(Debug)]
pub struct Message {
    pub content: Vec<Part>,
    pub role: String
} impl Message {
    pub fn get_real(&self) -> JsonValue {
        let mut obj = json::object! {
            "parts": [],
            "role": self.role.clone()
        };
        for i in self.content.clone() {
            obj["parts"].push(json::object! {
                "text": i.text
            }).unwrap()
        };
        obj
    }
}

/// TODO: Update this part to support different files
#[derive(Debug, Clone)]
pub struct Part {
    text: String,
}

impl Conversation {
    /// Creates a new conversation instance
    pub fn new(token: String, model: String) -> Self {
        Self {
            token,
            model,
            history: vec![],
            safety_settings: safety::default_safety_settings()
        }
    }

    /// Update the safety settings to different thresholds from [safety_settings]
    /// ## Example:
    /// ```rs 
    /// let mut convo = Conversation::new(
    ///     "ABC123".to_string,
    ///     "gemini-1.5-flash".to_string
    /// ).update_safety_settings(safety_settings::default());
    /// ```
    pub fn update_safety_settings(&mut self, settings: Vec<safety::SafetySetting>) {
        self.safety_settings = settings;
    }

    pub async fn prompt(&mut self, input: &str) -> String {
        match self.generate_content(vec![Part { text: input.to_string() }]).await {
            Ok(i) => i.get_text(),
            Err(e) => format!("Error: {0}", e)
        }
    }

    /// Sends a prompt to the Gemini API and returns the response
    pub async fn generate_content(&mut self, input: Vec<Part>) -> Result<GeminiResponse, GeminiError> {
        let model_verified = verify_model(&self.model, &self.token).await;
        if !model_verified {
            return Err(
                GeminiError::ModelError("Invalid Model. Please use get_models() to get a list of valid models".to_string())
            );
        }

        self.history.push(
            Message { content: input.clone(), role: "user".to_string() }
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
            data["contents"].push(i.get_real())?
        };
        for i in &self.safety_settings {
            data["safetySettings"].push(json::object! {
                "category": i.category.get_real(),
                "threshold": i.threshold.get_real()
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
        let candidate = response_dict["candidates"][0].clone();
        let token_count = response_dict["usageMetadata"]["candidatesTokenCount"]
            .as_u64()
            .ok_or_else(|| GeminiError::ParseError("Failed to extract token count".to_string()))?;
        let finish_reason = response::FinishReason::get_fake(candidate["finishReason"].as_str().unwrap());

        let parts_dict = candidate["content"]["parts"].clone();
        let mut content = vec![]; 
        for i in parts_dict.members() {
            let part = Part { text: i["text"].as_str().unwrap().to_string() };
            content.push(part)
        }

        let mut safety_rating = vec![];
        for i in candidate["safetyRatings"].members() {
            safety_rating.push(safety::SafetyRating {
                category: safety::HarmCategory::get_fake(
                    i["category"].as_str().unwrap()
                ),
                probability: safety::HarmProbability::get_fake(
                    i["probability"].as_str().unwrap()
                )
            })
        }

        self.history.push(
            Message { content: content.clone(), role: "model".to_string() }
        );

        Ok(GeminiResponse {
            content,
            safety_rating,
            token_count,
            finish_reason,
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

async fn verify_model(model_name: &str, token: &str) -> bool {
    let models = get_models(token).await.unwrap();
    models.contains(&model_name.to_string())
}
