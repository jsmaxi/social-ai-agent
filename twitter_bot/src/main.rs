use chrono::Local;
use llm_chain::{executor, parameters, prompt, step::Step};
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

async fn greet(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?; // New ChatGPT executor
    let step = Step::for_prompt_template(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}. Dont use any hashtags and don't use quotation marks."
    ));

    let res = step.run(&parameters!(name), &exec).await?;
    let content = res.to_immediate().await?.as_content();
    println!("{}", content);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tweet().await
    greet(String::from("JS")).await
}
