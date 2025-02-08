use std::sync::Arc;
use dotenvy::dotenv;
use std::env;
use serenity::{http::Http, model::user::CurrentUser};
use rusty_discord::discord::client::fetch_private_channels;

#[tokio::test]
async fn test_fetch_private_channels() {
    dotenv().ok(); 
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let http = Arc::new(Http::new(&token));

    let result = fetch_private_channels(&http).await;

    match result {
        Ok(private_channels) => {
            if private_channels.is_empty() {
                println!("✅ Test Passed: No private channels found.");
            } else {
                println!(
                    "✅ Test Passed: Fetched {} private channels: {:?}",
                    private_channels.len(),
                    private_channels.iter().map(|c| &c.id).collect::<Vec<_>>()
                );
            }
        }
        Err(err) => {
            panic!("❌ Test Failed: Error fetching private channels: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_fetch_bot_user() {
    dotenv().ok(); 

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set in .env.test file");
    let http = Arc::new(Http::new(&token));

    let result: serenity::Result<CurrentUser> = http.get_current_user().await;

    match result {
        Ok(user) => {
            println!("✅ Test Passed: Bot is logged in as '{}'", user.name);
        }
        Err(err) => {
            panic!("❌ Test Failed: Could not fetch bot user info: {:?}", err);
        }
    }
}

