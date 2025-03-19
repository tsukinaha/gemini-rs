#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        gemini_rs::chat("gemini-2.0-flash")
            .send_message("Explain how AI works")
            .await?
    );
    Ok(())
}
