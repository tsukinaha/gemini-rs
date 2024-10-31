use gemini_rs::Conversation;

#[tokio::main]
async fn main() {
    let convo = Conversation {
        token: &std::env::var("GEMINI_API_KEY").unwrap(),
        model: "gemini-1.5-flash",
    };
    let _ = convo.prompt("Hello what is ror2").await;
}
