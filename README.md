# STILL A WIP

A library to use Google Gemini's API directly in Rust!
Made because the current options weren't very capable and didn't support 100% of the official API.

## Example

```rust
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
```
