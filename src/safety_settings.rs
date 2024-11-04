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
}

enum HarmProbability {
    Unspecified,
    Neglibible,
    Low,
    Medium,
    High,
}

enum HarmBlockThreshold {
    Unspecified,
    LowAndAbove,
    MediumAndAbove,
    OnlyHigh,
    None,
    Off,
}

struct SafetySetting {
    category: HarmCategory,
    threshold: HarmBlockThreshold,
}

struct SafetyRating {
    category: HarmCategory,
    probability: HarmProbability,
}
