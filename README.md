# Social AI Agent

AI Agent bot that generates text messages and posts them on social media as configured.

This project is under development. Features may change.

---

## Functionality

The bot can:

- Summarize text and answer your questions
- Read text files (knowledge) from specified file path
- Post AI generated content to X (Twitter)
- Post scheduled X (Twitter) posts
- Expose web service endpoints for other systems
- Play a game by rules as AI opponent

---

## Technologies

- Rust

- X (Twitter) API

- LangChain LLM

- OpenAI API

---

## Commands

Run:

```console
cd twitter_bot
cargo build
cargo run
```

If you don't have Cargo and Rust installed on your machine: https://doc.rust-lang.org/cargo/getting-started/installation.html

Created new project:

```console
cargo new twitter_bot
```

---

## Dependencies

- twitter-v2 https://crates.io/crates/twitter-v2

- tokio https://crates.io/crates/tokio

- cron https://crates.io/crates/cron

- chrono https://crates.io/crates/chrono

- llm-chain https://crates.io/crates/llm-chain

- llm-chain-openai https://crates.io/crates/llm-chain-openai

- actix-web https://crates.io/crates/actix-web

- serde https://crates.io/crates/serde

- serde_json https://crates.io/crates/serde_json

---

## Environment Variables

- OPENAI_API_KEY (set it as local system environment variable)

Configure X access in [bot](./twitter_bot/src/bot.rs) file directly:

```
API_KEY: &str = "...";

API_KEY_SECRET: &str = "...";

ACCESS_TOKEN: &str = "...";

ACCESS_TOKEN_SECRET: &str = "...";
```

Run different examples in [main](./twitter_bot/src/main.rs) (comment and uncomment).

---

## X Access

Visit and sign up for free on X Developer Platform to obtain required access tokens and secrets:

https://developer.x.com/

X free plan currently offers:

- Get limited access to X's v2 API
- 1 environment
- Retrieve up to 100 Posts and 500 writes per month
- 17 requests / 24 hours PER USER
- 17 requests / 24 hours PER APP

This bot currently supports OAuth 1.0 protocol.

Generate new api token with secret, and access token with secret on the platform:

[Access](./images/twitter_access.png)

Copy-paste those values into the bot's configuration.

---

## LangChain LLM with OpenAI

To use LangChain LLM with ChatGPT, obtain OpenAI API key:

https://openai.com/api/

https://platform.openai.com/

Export OPENAI_API_KEY with obtained key value as environment variable on your system.

Note that OpenAI API is not free and requires you to purchase some credits to use it.

[Access](./images/openai-key.png)

[Billing](./images/openai-billing.png)

"$5 can cover about 2 million input or 500k output tokens." ($5 is more than enough for starting)

Pricing: https://openai.com/api/pricing/

Billing: https://platform.openai.com/settings/organization/billing/overview

Usage: https://platform.openai.com/settings/organization/usage

LLM OpenAI examples:

https://github.com/sobelio/llm-chain/blob/main/crates/llm-chain-openai/examples/generator_with_parameter.rs

---
