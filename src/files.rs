use reqwest::Method;

use crate::GeminiError;

#[derive(Debug, Clone)]
pub struct GeminiFile {
    pub file_uri: String,
    pub mime_type: String 
} impl GeminiFile {
    pub fn none() -> GeminiFile {
        GeminiFile {
            file_uri: "".to_string(),
            mime_type: "".to_string()
        }
    }
}

pub async fn upload_image<'a>(image_path: &'a str, api_key: &'a str) -> 
        Result<GeminiFile, GeminiError<'a>> {

    let file = std::fs::File::open(image_path)?;
    let file_size = file.metadata().unwrap().len();
    let file_type = image_path.rsplit_once('.').unwrap().1;
    let mime_filetype = get_mime_filetype(file_type);
    let url = format!(
        "https://generativelanguage.googleapis.com/upload/v1beta/files?key={0}",
        api_key
    );
    let file_name = image_path.split("/").last().unwrap().split_once(".").unwrap().0;
    let data = r#"{"file": {"display_name": ""#.to_owned() + file_name + r#""}}"#;

    let client = reqwest::Client::new();

    // Upload metadata to google servers
    let metadata_request = client
        .request(Method::POST, &url)
        .header("X-Goog-Upload-Protocol", "resumable")
        .header("X-Goog-Upload-Command", "start")
        .header("X-Goog-Upload-Header-Content-Length", file_size)
        .header("X-Goog-Upload-Header-Content-Type", &mime_filetype)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await
        .unwrap();

    let metadata_req_headers = metadata_request.headers();
    println!("{metadata_req_headers:?}\n");
    let upload_url = metadata_req_headers.get("x-goog-upload-url").unwrap().to_str().unwrap();

    // Upload the actual bytes
    let bytes_request = client
        .request(Method::POST, upload_url)
        .header("Content-Length", file_size)
        .header("X-Goog-Upload-Offset", 0)
        .header("X-Goog-Upload-Command", "upload, finalize")
        .body(std::fs::read(image_path).unwrap())
        .send()
        .await;

    println!("{0:?}\n", bytes_request.unwrap());

    // TEST
    let file_list_request = client
        .request(Method::GET, format!(
            "https://generativelanguage.googleapis.com/v1beta/files/?key={0}",
            std::env::var("GEMINI_API_KEY").unwrap()
        ))
        .send()
        .await
        .unwrap();

    let files_list = &json::parse(&file_list_request.text().await.unwrap()).unwrap()["files"];

    //println!("{0:?}\n", test_request.await.unwrap());

    Ok(GeminiFile{
        file_uri: files_list[0]["uri"].as_str().unwrap().to_string(),
        mime_type: mime_filetype
    })
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

