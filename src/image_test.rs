use reqwest::Method;

use crate::{Conversation, GeminiError};
use std::fs::File;

impl Conversation {
    pub async fn image_test(&self, image_path: &str) -> Result<(), GeminiError>{
        //let file = File::open(image_path).unwrap();
        //let file_size = file.metadata().unwrap().len();
        let url = format!(
            "https://storage.googleapis.com/upload/v1beta/files?key={0}",
            std::env::var("GEMINI_API_KEY").unwrap()
        );

        let client = reqwest::Client::new();
        let request = client
            .request(Method::POST, url)
            .header("X-Goog-Upload-Protocol", "resumable")
            .build();

        let http_response = client.execute(request?).await?;
        let response_dict = json::parse(&http_response.text().await.unwrap())?;
        println!("{0}", response_dict.dump());

        Ok(())
    }
}
