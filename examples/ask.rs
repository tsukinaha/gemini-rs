#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        gemini_rs::chat("gemini-1.5-flash")
            .send_message("Explain how AI works")
            .await?
    );
    Ok(())
}
