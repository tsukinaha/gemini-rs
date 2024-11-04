#[derive(Debug)]
pub enum HarmProbability {
    NotApplicable,
    Negligible,
    Low,
    Medium,
    High
}

#[derive(Debug)]
pub struct HarmProbabilities {
    pub harrasment: HarmProbability,
    pub hate_speech: HarmProbability,
    pub sexually_explicit: HarmProbability,
    pub dangerous_content: HarmProbability,
    pub civic_integrity: HarmProbability,
}
impl HarmProbabilities {
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0
        }
    }
}
pub struct Iter<'a> {
    inner: &'a HarmProbabilities,
    index: u8,
}
impl<'a> Iterator for Iter<'a> {
    type Item = (String, &'a HarmProbability);
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

pub fn none() -> HarmProbabilities {
    HarmProbabilities {
        harrasment: HarmProbability::NotApplicable,
        hate_speech: HarmProbability::NotApplicable,
        sexually_explicit: HarmProbability::NotApplicable,
        dangerous_content: HarmProbability::NotApplicable,
        civic_integrity: HarmProbability::NotApplicable,
    }
}

pub fn probability_from_str(input: &str) -> HarmProbability {
    match input {
        "LOW" => HarmProbability::Low,
        "MEDIUM" => HarmProbability::Medium,
        "HIGH" => HarmProbability::High,
        _ => HarmProbability::Negligible
    }
}
