#[derive(Clone, Debug)]
pub enum BlockThreshold {
    None,
    OnlyHigh,
    MediumAndAbove,
    LowAndAbove
}

#[derive(Debug)]
pub struct SafetySettings {
    harrasment: BlockThreshold,
    hate_speech: BlockThreshold,
    sexually_explicit: BlockThreshold,
    dangerous_content: BlockThreshold,
    civic_integrity: BlockThreshold
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
    type Item = &'a BlockThreshold;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.harrasment,
            1 => &self.inner.hate_speech,
            2 => &self.inner.sexually_explicit,
            3 => &self.inner.dangerous_content,
            4 => &self.inner.civic_integrity,
            _ => return None
        };
        self.index += 1;
        Some(ret)
    }
}

pub fn default() -> SafetySettings {
    SafetySettings {
        harrasment: BlockThreshold::LowAndAbove,
        hate_speech: BlockThreshold::LowAndAbove,
        sexually_explicit: BlockThreshold::LowAndAbove,
        dangerous_content: BlockThreshold::LowAndAbove,
        civic_integrity: BlockThreshold::LowAndAbove,
    }
}

pub fn from_threshold(threshold: BlockThreshold) -> SafetySettings {
    SafetySettings {
        harrasment: threshold.clone(),
        hate_speech: threshold.clone(),
        sexually_explicit: threshold.clone(),
        dangerous_content: threshold.clone(),
        civic_integrity: threshold.clone()
    }
}

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
