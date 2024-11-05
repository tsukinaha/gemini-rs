use crate::safety;

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
    SPII,
    /// Currently unused as function calling is not supported
    BadCall,
} impl FinishReason {
    pub fn get_fake(input: &str) -> FinishReason {
        match input {
            "STOP" => Self::Stop,
            "MAX_TOKENS" => Self::MaxTokens,
            "SAFETY" => Self::Safety,
            "RECITATION" => Self::Recitation,
            "LANGUAGE" => Self::Language,
            "OTHER" => Self::Other,
            "BLOCKLIST" => Self::Blocklist,
            "SPII" => Self::SPII,
            "MALFORMED_FUNCTION_CALL" => Self::BadCall,
            _ => Self::Unspecified,
        }
    }
}

/// Holds a response from Gemini
#[derive(Debug)]
pub struct Response {
    pub text: String,
    pub safety_rating: Vec<safety::SafetyRating>,
    pub token_count: u64,
    pub finish_reason: FinishReason,
}

