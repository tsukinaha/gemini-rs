use reqwest::Method;

use crate::Conversation;
use std::fs::File;

impl Conversation {
    pub async fn image_test(&self, image_path: &str) -> std::io::Result<()> {
        //let file = File::open(image_path).unwrap();
        //let file_size = file.metadata().unwrap().len();
        let url = format!(
            "https://storage.googleapis.com/upload/v1beta/files?key={0}",
            std::env::var("GEMINI_API_KEY").unwrap()
        );

        let client = reqwest::Client::new();
        let request = client
            .request(Method::POST, url)
            .header("X-Goog-Upload-Protocol", "resumable");

        Ok(())
    }
}
