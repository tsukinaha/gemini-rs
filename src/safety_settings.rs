/// Block thresholds for different areas of Gemini's responses
#[derive(Clone, Debug)]
pub enum BlockThreshold {
    /// Always show regardless of probability of unsafe content
    None,
    /// Block when high probability of unsafe content
    OnlyHigh,
    /// Block when medium or high probability of unsafe content
    MediumAndAbove,
    /// Block when low, medium or high probability of unsafe content **(default)**
    LowAndAbove
}
impl BlockThreshold {
    pub fn get_actual(&self) -> String {
        let actual = match self {
            Self::None => "BLOCK_NONE",
            Self::OnlyHigh => "BLOCK_ONLY_HIGH",
            Self::MediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
            Self::LowAndAbove => "BLOCK_LOW_AND_ABOVE"
        };
        actual.to_string()
    }
}

/// Safety settings for Gemini's responses
#[derive(Debug)]
pub struct SafetySettings {
    /// Negative or harmful comments targeting identity and/or protected attributes
    pub harrasment: BlockThreshold,
    /// Content that is rude, disrespectful, or profane
    pub hate_speech: BlockThreshold,
    /// Contains references to sexual acts or other lewd content
    pub sexually_explicit: BlockThreshold,
    /// Promotes, facilitates, or encourages harmful acts
    pub dangerous_content: BlockThreshold,
    /// Election-related queries
    pub civic_integrity: BlockThreshold
}
impl SafetySettings {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0
        }
    }
}
pub struct Iter<'a> {
    inner: &'a SafetySettings,
    index: u8,
}
impl<'a> Iterator for Iter<'a> {
    type Item = (String, &'a BlockThreshold);
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => ("HARM_CATEGORY_HARASSMENT".to_string(), &self.inner.harrasment),
            1 => ("HARM_CATEGORY_HATE_SPEECH".to_string(), &self.inner.hate_speech),
            2 => ("HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(), &self.inner.sexually_explicit),
            3 => ("HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),&self.inner.dangerous_content),
            4 => ("HARM_CATEGORY_CIVIC_INTEGRITY".to_string(), &self.inner.civic_integrity),
            _ => return None
        };
        self.index += 1;
        Some(ret)
    }
}

/// Returns an instance of [SafetySettings] with default [BlockThreshold]s (everything on `LowAndAbove`)
pub fn default() -> SafetySettings {
    SafetySettings {
        harrasment: BlockThreshold::LowAndAbove,
        hate_speech: BlockThreshold::LowAndAbove,
        sexually_explicit: BlockThreshold::LowAndAbove,
        dangerous_content: BlockThreshold::LowAndAbove,
        civic_integrity: BlockThreshold::LowAndAbove,
    }
}

/// Returns an instance of [SafetySettings] where all fields have 
/// the same [BlockThreshold]
pub fn from_threshold(threshold: BlockThreshold) -> SafetySettings {
    SafetySettings {
        harrasment: threshold.clone(),
        hate_speech: threshold.clone(),
        sexually_explicit: threshold.clone(),
        dangerous_content: threshold.clone(),
        civic_integrity: threshold.clone()
    }
}

/// Returns an instance of [SafetySettings] with custom [BlockThreshold]s
pub fn custom(
    harrasment: BlockThreshold,
    hate_speech: BlockThreshold,
    sexually_explicit: BlockThreshold,
    dangerous_content: BlockThreshold,
    civic_integrity: BlockThreshold
) -> SafetySettings {
    SafetySettings{
        harrasment,
        hate_speech,
        sexually_explicit,
        dangerous_content,
        civic_integrity
    }
}


