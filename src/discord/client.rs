use std::sync::Arc;

use serenity::{
    async_trait,
    client::Client,
    framework::standard::{macros::group, StandardFramework},
    http::Http,
    model::{channel::PrivateChannel, gateway::Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Result as SerenityResult,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[group]
struct General;

pub async fn start_discord_client(token: &str) {
    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // Set the bot's prefix

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create Discord client");

    tokio::spawn(async move {
        if let Err(err) = client.start().await {
            eprintln!("Error running Discord client: {:?}", err);
        }
    });
}

pub async fn fetch_private_channels(
    http: &Arc<Http>,
) -> serenity::Result<Vec<Arc<PrivateChannel>>> {
    match http.get_user_dm_channels().await {
        Ok(private_channels) => {
            if private_channels.is_empty() {
                eprintln!("⚠️ API Response: No private channels found.");
            } else {
                eprintln!(
                    "✅ API Response: Found {} private channels: {:?}",
                    private_channels.len(),
                    private_channels.iter().map(|c| &c.id).collect::<Vec<_>>()
                );
            }
            Ok(private_channels.into_iter().map(Arc::new).collect())
        }
        Err(err) => {
            eprintln!("❌ API Error: {:?}", err);
            Err(err)
        }
    }
}
