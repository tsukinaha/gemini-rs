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
        let file_name = image_path.split("/").last().unwrap();
        let data = r#"{"file": {"display_name": ""#.to_owned() + file_name + r#""}}"#;

        let client = reqwest::Client::new();

        // Upload metadata to google servers
        let metadata_request = client
            .request(Method::POST, &url)
            .header("X-Goog-Upload-Protocol", "resumable")
            .header("X-Goog-Upload-Command", "start")
            .header("X-Goog-Upload-Header-Content-Length", file_size)
            .header("X-Goog-Upload-Header-Content-Type", mime_filetype)
            .header("Content-Type", "application/json")
            .body(data)
            .send();

        Self::print_json(metadata_request.await.unwrap()).await;

        // Upload the actual bytes
        let bytes_request = client
            .request(Method::POST, &url)
            .header("Content-Length", file_size)
            .header("X-Goog-Upload-Offset", 0)
            .header("X-Goog-Upload-Command", "upload, finalize")
            .body(fs::read(image_path).unwrap())
            .send();

        println!("{0:?}\n", bytes_request.await.unwrap());

        // TEST
        let test_request = client
            .request(Method::GET,
                "https://generativelanguage.googleapis.com/v1beta/files/".to_owned()
                + file_name
            )
            .send();

        println!("{0:?}\n", test_request.await.unwrap());

        Ok(())
    }

    async fn print_json(input: reqwest::Response) {
        let input_text = input.text().await;
        println!("{input_text:?}");
        //let input_json = json::parse(&input_text).unwrap();
        //println!("{0}\n", input_json.dump());
    }

    fn get_mime_filetype(input: &str) -> String {
        const IMAGES: [&str; 3] = [
            "jpeg",
            "png",
            "webp",
        ];
        if IMAGES.contains(&input) {
            format!("image/{0}", input)
        } else {
            "a".to_string()
        }
    }
}
