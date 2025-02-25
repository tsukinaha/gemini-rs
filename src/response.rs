use crate::{safety, Part};

#[derive(Debug)]
pub enum FinishReason {
    Unspecified,
    Stop,
    MaxTokens,
    Safety,
    Recitation,
    Language,
    Other,
    Blocklist,
    Prohibited,
    Spii,
    /// Currently unused as function calling is not supported
    BadCall,
}
impl FinishReason {
    pub fn get_fake(input: &str) -> FinishReason {
        match input {
            "STOP" => Self::Stop,
            "MAX_TOKENS" => Self::MaxTokens,
            "SAFETY" => Self::Safety,
            "RECITATION" => Self::Recitation,
            "LANGUAGE" => Self::Language,
            "OTHER" => Self::Other,
            "BLOCKLIST" => Self::Blocklist,
            "SPII" => Self::Spii,
            "MALFORMED_FUNCTION_CALL" => Self::BadCall,
            _ => Self::Unspecified,
        }
    }
}

/// Holds a response from Gemini
#[derive(Debug)]
pub struct GeminiResponse {
    pub content: Vec<Part>,
    pub safety_rating: Vec<safety::SafetyRating>,
    pub token_count: u64,
    pub finish_reason: FinishReason,
}
impl GeminiResponse {
    pub fn get_text(&self) -> String {
        //self.content[0].text.clone()
        if let Part::Text(text) = &self.content[0] {
            return text.to_string();
        };
        "".to_string()
    }
}
