use std::{
    fs::File,
    io::{Read, Write},
};

use crate::{Conversation, Message, Part};

impl Conversation {
    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        let mut json = json::object! {
            "history": []
        };
        for i in self.history.iter() {
            let mut content = vec![];
            for part in i.content.iter() {
                content.push(match part {
                    Part::Text(text) => json::object! {"text": *text.clone()},
                    Part::File(file_data) => json::object! {
                        "file_uri": file_data.file_uri.clone(),
                        "mime_type": file_data.mime_type.clone()
                    },
                })
            }
            json["history"]
                .push(json::object! {
                    "role": i.role.clone(),
                    "content": content
                })
                .unwrap();
        }
        let _ = file.write_all(json.dump().as_bytes());
    }

    pub fn load(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let json = json::parse(&contents).unwrap();
        let mut history: Vec<Message> = vec![];
        for i in json["history"].members() {
            let mut parts = vec![];
            for part in i["content"].members() {
                if part.has_key("text") {
                    parts.push(Part::Text(part["text"].as_str().unwrap().to_string()));
                } else if part.has_key("file_uri") {
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
