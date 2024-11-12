use reqwest::Method;

use crate::{Conversation, GeminiError};
use std::fs::{self, File};

impl Conversation {
    pub async fn image_test(&self, image_path: &str) -> Result<(), GeminiError>{
        let file = File::open(image_path)?;
        let file_size = file.metadata().unwrap().len();
        let file_type = image_path.rsplit_once('.').unwrap().1;
        let mime_filetype = Self::get_mime_filetype(file_type);
        let url = format!(
            "https://generativelanguage.googleapis.com/upload/v1beta/files?key={0}",
            std::env::var("GEMINI_API_KEY").unwrap()
        );

        let client = reqwest::Client::new();

        // Upload metadata to google servers
        let metadata_request = client
            .request(Method::POST, &url)
            .header("X-Goog-Upload-Protocol", "resumable")
            .header("X-Goog-Upload-Command", "start")
            .header("X-Goog-Upload-Header-Content-Length", file_size)
            .header("X-Goog-Upload-Header-Content-Type", mime_filetype)
            .header("Content-Type", "application/json")
            .body(r#""file": {"display_name": ""#.to_owned() + image_path.rsplit("/").last().unwrap() + "}}")
            .send();

        println!("{0:?}", metadata_request.await.unwrap());

        // Upload the actual bytes
        let bytes_request = client
            .request(Method::POST, &url)
            .header("Content-Length", file_size)
            .header("X-Goog-Upload-Offset", 0)
            .header("X-Goog-Upload-Command", "upload, finalize")
            .body(fs::read(image_path).unwrap())
            .send();

        println!("{0:?}", bytes_request.await.unwrap());



        Ok(())
    }

    fn get_mime_filetype(input: &str) -> String {
        const IMAGES: [&str; 3] = [
            "jpeg",
            "png",
            "webp",
        ];
        if IMAGES.contains(&input) {
            format!("images/{0}", input)
        } else {
            "a".to_string()
        }
    }
}
