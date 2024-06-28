use std::env;
use serde::Deserialize;
use reqwest::{Client, header::AUTHORIZATION};
const DISCORD_API_ROOT: &str = "https://discord.com/api/v10/";
#[derive(Deserialize)]
pub struct DiscordUser {
    id: Option<String>,
    username: Option<String>,
    avatar: Option<String>,
    discriminator: Option<String>,
}

pub async fn validate_discord_token() {
    // ensure that the token is set
    if env::var("DISCORD_TOKEN").ok().is_none() {
        panic!("DISCORD_TOKEN is not set! It is required in order to fetch the profile pictures")
    }

    // ensure that the token is valid
    let body;
    let res = Client::new().get(format!("{DISCORD_API_ROOT:}users/@me"))
        .header(AUTHORIZATION, format!("Bot {}", env::var("DISCORD_TOKEN").unwrap()))
        .send()
        .await;
    match res {
        Ok(resp) => {
            body = resp.json::<DiscordUser>().await.expect("Unable to reach the discord API");
        }
        Err(err) => {
            panic!("Unable to reach the discord API: {err}")
        }
    }

    if body.id.is_none() {
        println!("BOT {}", env::var("DISCORD_TOKEN").unwrap());
        panic!("Invalid DISCORD_TOKEN. It is required in order to fetch the profile pictures")
    }

    log::info!("Successfully logged in as {}#{}.", body.username.unwrap(), body.discriminator.unwrap())
}