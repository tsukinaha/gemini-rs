#[tokio::main]
async fn main() {
    let result = reqwest::get("https://api.spotify.com/v1/search").await;
    println!("{result:?}")
}
