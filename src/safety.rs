//! Handles everything related to safety 
//!
//! Is used to change what kind of messages will be blocked, as well as to show why
//! a message was blocked

/// The category of a [SafetyRating]
#[derive(Debug)]
pub enum HarmCategory {
    /// Category is unspecified
    Unspecified,
    /// **PaLM** - Negative or harmful comments targeting identity and/or protected attribute
    Derogatory,
    /// **PaLM** - Content that is rude, disrespectful, or profane
    Toxicity,
    /// **PaLM** - Describes scenarios depicting violence against an individual or group, or general descriptions of gore
    Violence,
    /// **PaLM** - Contains references to sexual acts or other lewd content
    Sexual,
    /// **PaLM** - Promotes unchecked medical advice
    Medical,
    /// **PaLM** - Dangerous content that promotes, facilitates, or encourages harmful acts
    Dangerous,
    /// **Gemini** - Harassment content
    Harassment,
    /// **Gemini** - Hate speech and content
    HateSpeech,
    /// **Gemini** - Sexually explicit content
    SexuallyExplicit,
    /// **Gemini** - Dangerous content
    DangerousContent,
    /// **Gemini** - Content that may be used to harm civic integrity
    CivicIntergrity,
} impl HarmCategory {
    pub fn get_real(&self) -> &str {
        match self {
            Self::Unspecified => "HARM_CATEGORY_UNSPECIFIED",
            Self::Derogatory => "HARM_CATEGORY_DEROGATORY",
            Self::Toxicity => "HARM_CATEGORY_TOXICITY",
            Self::Violence => "HARM_CATEGORY_VIOLENCE",
            Self::Sexual => "HARM_CATEGORY_SEXUAL",
            Self::Medical => "HARM_CATEGORY_MEDICAL",
            Self::Dangerous => "HARM_CATEGORY_DANGEROUS",
            Self::Harassment => "HARM_CATEGORY_HARASSMENT",
            Self::HateSpeech => "HARM_CATEGORY_HATE_SPEECH",
            Self::SexuallyExplicit => "HARM_CATEGORY_SEXUALLY_EXPLICIT",
            Self::DangerousContent => "HARM_CATEGORY_DANGEROUS_CONTENT",
            Self::CivicIntergrity => "HARM_CATEGORY_CIVIC_INTEGRITY",
        }
    }
    pub fn get_fake(input: &str) -> HarmCategory {
        match input {
            "HARM_CATEGORY_DEROGATORY" => HarmCategory::Derogatory,
            "HARM_CATEGORY_TOXICITY" => HarmCategory::Toxicity,
            "HARM_CATEGORY_VIOLENCE" => HarmCategory::Violence, 
            "HARM_CATEGORY_SEXUAL" => HarmCategory::Sexual,
            "HARM_CATEGORY_MEDICAL" => HarmCategory::Medical,
            "HARM_CATEGORY_DANGEROUS" => HarmCategory::Dangerous,
            "HARM_CATEGORY_HARASSMENT" => HarmCategory::Harassment,
            "HARM_CATEGORY_HATE_SPEECH" => HarmCategory::HateSpeech,
            "HARM_CATEGORY_SEXUALLY_EXPLICIT" => HarmCategory::SexuallyExplicit,
            "HARM_CATEGORY_DANGEROUS_CONTENT" => HarmCategory::DangerousContent,
            "HARM_CATEGORY_CIVIC_INTEGRITY" => HarmCategory::CivicIntergrity,
            _ => HarmCategory::Unspecified,
        }
    }
}

/// The probability that a piece of content is harmful
///
/// The classification system gives the probability of the content being unsafe.
/// This does not indicate the severity of harm for a piece of content.
#[derive(Debug)]
pub enum HarmProbability {
    /// Probability is unspecified
    Unspecified,
    /// Content has a negligible chance of being unsafe
    Negligible,
    /// Content has a low chance of being unsafe
    Low,
    /// Content has a medium chance of being unsafe
    Medium,
    /// Content has a high chance of being unsafe
    High,
} impl HarmProbability {
    pub fn get_real(&self) -> &str {
        match self {
            Self::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
            Self::Negligible => "NEGLIGIBLE",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
        }
    }
    pub fn get_fake(input: &str) -> HarmProbability {
        match input {
            "NEGLIGIBLE" => HarmProbability::Negligible,
            "LOW" => HarmProbability::Low,
            "MEDIUM" => HarmProbability::Medium,
            "HIGH" => HarmProbability::High,
            _ => HarmProbability::Unspecified,
        }
    }
}

/// Block at and beyond a specified harm probability
#[derive(Debug, Clone)]
pub enum HarmBlockThreshold {
    /// Threshold is unspecified
    Unspecified,
    /// Content with [HarmProbability::Negligible] will be allowed.
    LowAndAbove,
    /// Content with [HarmProbability::Negligible] and [HarmProbability::Low] will be allowed.
    MediumAndAbove,
    /// Content with [HarmProbability::Negligible], [HarmProbability::Low], and [HarmProbability::Medium] will be allowed.
    OnlyHigh,
    /// All content will be allowed.
    None,
    /// Turn off the safety filter.
    Off,
} impl HarmBlockThreshold {
    pub fn get_real(&self) -> &str {
        match self {
            Self::Unspecified => "HARM_BLOCK_THRESHOLD_UNSPECIFIED",
            Self::LowAndAbove => "BLOCK_LOW_AND_ABOVE",
            Self::MediumAndAbove => "BLOCK_MEDIUM_AND_ABOVE",
            Self::OnlyHigh => "BLOCK_ONLY_HIGH",
            Self::None => "BLOCK_NONE",
            Self::Off => "OFF",
        }
    }
}

/// Safety setting, affecting the safety-blocking behavior.
/// 
/// Passing a safety setting for a category changes the allowed probability that content is blocked.
#[derive(Debug)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

/// Safety rating for a piece of content.
/// 
/// The safety rating contains the category of harm and the harm probability level in that category for a piece of content.
/// Content is classified for safety across a number of harm categories
/// and the probability of the harm classification is included here.
#[derive(Debug)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

// TODO: add a custom safety settings func for setting individual fields
pub fn safety_settings_from(threshold: HarmBlockThreshold) -> Vec<SafetySetting> {
    vec![
        SafetySetting {
            category: HarmCategory::Harassment,
            threshold: threshold.clone()
        },
        SafetySetting {
            category: HarmCategory::HateSpeech,
            threshold: threshold.clone()
        },
        SafetySetting {
            category: HarmCategory::SexuallyExplicit,
            threshold: threshold.clone()
        },
        SafetySetting {
            category: HarmCategory::DangerousContent,
            threshold: threshold.clone()
        },
        SafetySetting {
            category: HarmCategory::CivicIntergrity,
            threshold: threshold.clone()
        },
    ]
}

pub fn default_safety_settings() -> Vec<SafetySetting> {
    safety_settings_from(HarmBlockThreshold::LowAndAbove)
}
