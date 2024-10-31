//!HI CHAT

use std::{
    fs::File,
    io::{Result, Write},
};

use json::JsonValue;
//use crate::settings;
use reqwest::{Client, Error, Method};

pub struct Conversation {
    pub token: String,
    pub model: String,
    //history
}

pub struct Response {
    pub text: String,
}

impl Conversation {
    /// Used to contact the Gemini API
    pub async fn prompt(&self, input: &str) -> Result<Response, Error> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{0}:generateContent?key={1}",
            self.model, self.token
        );
        let data = "{
        \"contents\": [{
        \"parts\": [{\"text\": \""
            .to_owned()
            + input
            + "\"}]
        }] 
    }";

        let client = Client::new();
        let request = client
            .request(Method::POST, url)
            .header("Content-Type", "application/json")
            .body(data)
            .build();
        let http_response = client.execute(request?).await?;
        let response_json = http_response.text().await?;
        let response_dict = json::parse(&response_json).unwrap();
        log_json(&response_dict);
        let response_text = response_dict["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap();
        let response = Response {
            text: response_text.to_string(),
        };
        Ok(response)
    }
}

fn log_json(json: &JsonValue) -> Result<()> {
    let mut file = File::create("log.json")?;
    file.write_all(json.as_str().unwrap().as_bytes())?;
    Ok(())
}
