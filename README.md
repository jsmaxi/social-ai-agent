# Social AI Agent

AI Agent bot that generates text messages and posts them on social media as configured.

Written with Rust.

---

## Commands

Created:

```console
cargo new twitter_bot
```

Run:

```console
cd twitter_bot
cargo build
cargo run
```

---

## Dependencies

- twitter-v2 https://crates.io/crates/twitter-v2

- tokio https://crates.io/crates/tokio

- cron https://crates.io/crates/cron

- chrono https://crates.io/crates/chrono

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
