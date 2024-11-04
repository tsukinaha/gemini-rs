#[derive(Debug)]
enum HarmCategory {
    Unspecified,
    Derogatory,
    Toxicity,
    Violence,
    Sexual,
    Medical,
    Dangerous,
    Harassment,
    HateSpeech,
    SexuallyExplicit,
    DangerousContent,
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
}

#[derive(Debug)]
enum HarmProbability {
    Unspecified,
    Neglibible,
    Low,
    Medium,
    High,
} impl HarmProbability {
    pub fn get_real(&self) -> &str {
        match self {
            Self::Unspecified => "HARM_PROBABILITY_UNSPECIFIED",
            Self::Neglibible => "NEGLIGIBLE",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
        }
    }
}

#[derive(Debug, Clone)]
enum HarmBlockThreshold {
    Unspecified,
    LowAndAbove,
    MediumAndAbove,
    OnlyHigh,
    None,
    Off,
}

#[derive(Debug)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

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
