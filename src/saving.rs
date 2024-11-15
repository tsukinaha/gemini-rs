use std::fs::File;

use crate::{Conversation, GeminiError, Part};

impl Conversation {
    pub fn save(&self, path: &str) -> Result<(), GeminiError> {
        let mut file = File::create(path)?;
        let mut json = json::object! {
            "history": []
        };
        for i in self.history.iter() {
            let content = vec![];
            for part in i.content.iter() {
                content.push(match part {
                    Part::Text(text) => json::object! {"text": text}
                })
            };
            json["history"].push(json::object! {
                "role": i.role.clone(),
                "content": content
            })?;
        }
        Ok(())
        //file.write_all()
    }
}
