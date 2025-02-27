#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = gemini_rs::Client::instance();
    let models = client.models().await?;
    println!("{models:#?}");
    Ok(())
}
