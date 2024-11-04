#[derive(Debug)]
pub enum HarmCategory {
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
pub fn get_fake_harm_category(input: &str) -> HarmCategory {
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

#[derive(Debug)]
pub enum HarmProbability {
    Unspecified,
    Negligible,
    Low,
    Medium,
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
}
pub fn get_fake_harm_probability(input: &str) -> HarmProbability {
    match input {
        "NEGLIGIBLE" => HarmProbability::Negligible,
        "LOW" => HarmProbability::Low,
        "MEDIUM" => HarmProbability::Medium,
        "HIGH" => HarmProbability::High,
        _ => HarmProbability::Unspecified,
    }
}

#[derive(Debug, Clone)]
pub enum HarmBlockThreshold {
    Unspecified,
    LowAndAbove,
    MediumAndAbove,
    OnlyHigh,
    None,
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
