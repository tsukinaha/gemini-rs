enum FinishReason {
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
}

pub struct Response {
    text: String,
    finish_reason: FinishReason,
}
