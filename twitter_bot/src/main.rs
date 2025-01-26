use chrono::Local;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

fn content() -> String {
    let tweet_content: String = format!(
        "Hello! This is my daily post at {}. Expect more 🫡",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    println!("Text: {:?}", tweet_content);

    tweet_content
}

fn oauth() -> Oauth1aToken {
    /* Bot uses OAuth 1.0 Visit: https://developer.x.com */

    let api_key: &str = "...";
    let api_key_secret: &str = "...";
    let access_token: &str = "...";
    let access_token_secret: &str = "...";

    let token: Oauth1aToken = Oauth1aToken::new(
        &api_key,
        &api_key_secret,
        &access_token,
        &access_token_secret,
    );

    token
}

async fn tweet() -> Result<(), Box<dyn std::error::Error>> {
    let response = TwitterApi::new(oauth())
        .post_tweet()
        .text(content())
        .send()
        .await;

    match response {
        Ok(_) => println!("Tweet posted successfully!"),
        Err(e) => println!("Response: {:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tweet().await
}
