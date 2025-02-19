mod bot;
mod game;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Web Server not awaited intentionally
    // _ = server::init();
    // bot::tweet(None).await?;
    // bot::greet(String::from("Joe")).await?;
    // bot::summarize().await?;
    // bot::tweet_scheduled_with_retry(String::from("data/knowledge_universe.txt"), None).await?;
    // bot::summarize_from_file(String::from("data/knowledge_quantum.txt")).await?;
    // bot::tweet_from_file_knowledge(String::from("data/knowledge_universe.txt")).await?;
    // bot::answer_from_file(String::from("data/knowledge_sentinel.txt")).await?;
    // game::play("./data/rules.json").await;
    bot::tweet_scheduled_with_retry(
        String::from("data/knowledge_universe.txt"),
        Some(String::from("0 */1 * * * ?")), // post every minute
    )
    .await?;
    Ok(())
}
