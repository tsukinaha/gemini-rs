use gemini_rs::Conversation;

#[tokio::main]
async fn main() {
    let mut convo = Conversation::new(
        std::env::var("GEMINI_API_KEY").unwrap(),
        "gemini-1.5-flash".to_string()
    );
    //let convo = Conversation {
    //    token: std::env::var("GEMINI_API_KEY").unwrap(),
    //    model: "gemini-1.5-flash".to_string(),
    //};
    let a = convo.prompt("If you had to describe risk of rain 2 in one word, what word would it be? dont say anything more").await.unwrap();
    println!("{a:?}");
}
