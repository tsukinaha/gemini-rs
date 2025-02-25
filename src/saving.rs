use std::{
    fs::File,
    io::{Read, Write},
};
use serde_json::{json, Value};

use crate::{Conversation, Message, Part};

impl Conversation {
    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let mut history = Vec::new();
        for i in &self.history {
            let mut content = vec![];
            for part in &i.content {
                content.push(match part {
                    Part::Text(text) => json!({ "text": text }),
                    Part::File(file_data) => json!({
                        "file_uri": file_data.file_uri,
                        "mime_type": file_data.mime_type
                    }),
                });
            }
            history.push(json!({
                "role": i.role,
                "content": content
            }));
        }
        let json = json!({ "history": history });
        let _ = file.write_all(serde_json::to_string(&json).unwrap().as_bytes());
    }

    pub fn load(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let json: Value = serde_json::from_str(&contents).unwrap();
        let mut history: Vec<Message> = vec![];
        for i in json["history"].as_array().unwrap() {
            let mut parts = vec![];
            for part in i["content"].as_array().unwrap() {
                if part.get("text").is_some() {
                    parts.push(Part::Text(part["text"].as_str().unwrap().to_string()));
                } else if part.get("file_uri").is_some() {
                    parts.push(Part::File(crate::files::GeminiFile {
                        file_uri: part["file_uri"].as_str().unwrap().to_string(),
                        mime_type: part["mime_type"].as_str().unwrap().to_string(),
                    }));
                }
            }
            history.push(Message {
                content: parts,
                role: i["role"].as_str().unwrap().to_string(),
            });
        }
        self.history = history;
    }
}
