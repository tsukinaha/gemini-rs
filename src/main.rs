use gemini_rs::Conversation;

#[tokio::main]
async fn main() {
    let mut convo = Conversation::new(
        std::env::var("GEMINI_API_KEY").unwrap(),
        "gemini-1.5-flash".to_string()
    );

    let a = convo.prompt("If you had to describe risk of rain 2 in one word, what word would it be? dont say anything more").await.unwrap();
    println!("{0:?}", a.text);
    let b = convo.prompt("Now explain your reasoning").await.unwrap();
    println!("{0:?}", b.text);
}
