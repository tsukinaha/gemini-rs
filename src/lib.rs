//!HI CHAT

use reqwest::Error;
///# Heading 1
///## Heading 2
///### Heading 3
pub async fn prompt(input: &str) -> Result<String, Error>{
    let token = std::env::var("GEMINI_API_KEY").unwrap();
    let model = "gemini-1.5-flash";
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={token}");
    Ok(reqwest::get(url).await?.text().await?)
}
