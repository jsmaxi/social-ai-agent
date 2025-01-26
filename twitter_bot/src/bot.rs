use chrono::{DateTime, Local, Utc};
use cron::Schedule;
use llm_chain::prompt;
use llm_chain::{chains::sequential::Chain, executor, parameters, prompt::Data, step::Step};
use std::fs;
use std::str::FromStr;
use std::thread;
use twitter_v2::{authorization::Oauth1aToken, TwitterApi};

// X (Twitter) access configuration
const API_KEY: &str = "";
const API_KEY_SECRET: &str = "";
const ACCESS_TOKEN: &str = "";
const ACCESS_TOKEN_SECRET: &str = "";

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

    let token: Oauth1aToken = Oauth1aToken::new(
        &API_KEY,
        &API_KEY_SECRET,
        &ACCESS_TOKEN,
        &ACCESS_TOKEN_SECRET,
    );

    token
}

pub async fn tweet_scheduled_with_retry(
    file_path: String,
    schedule_expression: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Define the cron expression for daily posting
    // Some parsers expect only 5 fields (standard unix cron), while others may allow 6 or 7 fields.
    // Note: Quartz cron expressions have six fields instead of five, with an optional seventh field for specifying the year.
    // UNIX cron expression order: MINUTES HOURS DAY_OF_MONTH MONTH DAY_OF_WEEK
    // Quartz cron expression order: Seconds Minutes Hours Day_of_month Month Day_of_week [Year]
    // Examples:
    // every 10 minutes = "*/10 * * * *" OR "0 */10 * * * ?"
    // every hour = "0 * * * *" OR "0 0 * * * ?"
    // every half a day = "0 */12 * * *" OR "0 0 */12 * * ?"
    // every day at 10 PM = "0 22 * * *" OR "0 0 22 * * ?"
    // at the start of every hour = "0 * * * *" OR "0 0 * * * ?"
    // every sunday at midnight = "0 0 * * 0" OR "0 0 0 ? * SUN"

    let expression: String = match schedule_expression {
        Some(s) => s,
        None => String::from("0 */5 * * * ?"), // Default - every 5 minutes
    };

    let schedule: Schedule =
        Schedule::from_str(&expression).expect("Failed to parse CRON expression");

    loop {
        let now_utc: DateTime<Utc> = Utc::now();
        if let Some(next) = schedule.upcoming(Utc).take(1).next() {
            println!("Upcoming tweet time: {}", next);

            let until_next = next - now_utc;
            thread::sleep(until_next.to_std().unwrap());

            // Attempt to post the tweet with a retry mechanism
            let mut attempts: usize = 0;
            const MAX_ATTEMPTS: usize = 3;

            while attempts < MAX_ATTEMPTS {
                match tweet_from_file_knowledge(file_path.clone()).await {
                    Ok(_) => {
                        break; // Exit the while loop if OK
                    }
                    Err(e) => {
                        attempts += 1;
                        println!(
                            "Failed to post tweet: {}. Attempt {}/{}",
                            e, attempts, MAX_ATTEMPTS
                        );
                        if attempts == MAX_ATTEMPTS {
                            println!("Maximum attempts reached. Giving up on posting this tweet.");
                        } else {
                            println!("Retrying...");
                        }
                    }
                }
            }
        }
    }
}

pub async fn tweet(text: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let content: String = match text {
        Some(t) => t,
        None => content(), // Default content to post if none
    };

    let response = TwitterApi::new(oauth())
        .post_tweet()
        .text(content)
        .send()
        .await;

    if let Err(e) = response {
        println!("Error response: {:?}", e);
        return Err(Box::new(e));
    } else {
        println!("Tweet posted successfully!");
        return Ok(());
    }
}

pub async fn greet(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?; // New ChatGPT executor
    let step = Step::for_prompt_template(prompt!(
        "You are a bot for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}. Dont use any hashtags and dont use quotation marks."
    ));

    let res = step.run(&parameters!(name), &exec).await?;
    let content = res.to_immediate().await?.as_content();
    println!("{}", content);

    Ok(())
}

pub async fn summarize() -> Result<(), Box<dyn std::error::Error>> {
    let exec = executor!()?; // New ChatGPT executor

    let first_step = Step::for_prompt_template(prompt!(
        "You are an assistant that provides information.",
        "What is the capital of USA?"
    ));

    let second_step = Step::for_prompt_template(prompt!(
        "You are an assistant that summarizes information.",
        "Summarize this answer: {{text}}"
    ));

    let chain: Chain = Chain::new(vec![first_step, second_step]);

    let res = chain
        .run(parameters!("text" => "The capital is..."), &exec)
        .await?;

    let content = res.to_immediate().await?.as_content();

    println!("{}", content);

    Ok(())
}

pub async fn summarize_from_file(file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let knowledge: String = fs::read_to_string(file_path)?;

    let exec = executor!()?; // New ChatGPT executor
    let step = Step::for_prompt_template(prompt!(
        "Context:\n{{text}}\n\nQuestion: What key insights can you extract from this context?"
    ));

    let res = step.run(&parameters!(knowledge), &exec).await?;
    let content = res.to_immediate().await?.as_content();
    println!("{}", content);

    Ok(())
}

fn remove_prefix_from_data(data: Data<String>, prefix: &str) -> String {
    let data_str: String = data.to_string();
    data_str
        .strip_prefix(prefix)
        .unwrap_or(&data_str)
        .to_string()
}

pub async fn tweet_from_file_knowledge(
    file_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let knowledge: String = fs::read_to_string(file_path)?;

    let exec = executor!()?; // New ChatGPT executor
    let step = Step::for_prompt_template(prompt!(
        "Context:\n{{text}}\n\nTask: create a post based on unique insights of the context. Make it fun. Maximum 280 characters. Dont use any hashtags and dont use quotation marks."
    ));

    let res = step.run(&parameters!(knowledge), &exec).await?;
    let content = res.to_immediate().await?.as_content();

    let prefix = "Assistant: ";
    let post = remove_prefix_from_data(content, &prefix);

    println!("Generated post. {}", post);

    tweet(Some(post)).await?;

    Ok(())
}
