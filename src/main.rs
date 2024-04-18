use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity};
use sea_orm::{ConnectOptions, Database};
use std::env::var;
use std::time::Duration;
use teianbot::{
    commands::{body::*, hello::*},
    event::{message::*, voice_state_update::*},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let discord_token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let database_uri = var("DATABASE_URL")
        .expect("Missing `DATABASE_URL` env var, see README for more information.");
    let mut opt = ConnectOptions::new(database_uri);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .set_schema_search_path("teianbo");
    let db = match Database::connect(opt).await {
        Ok(cn) => cn,
        Err(_) => panic!(),
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { connection: db })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(
        discord_token,
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            update_chatcount(ctx, data, new_message).await
        }
        serenity::FullEvent::VoiceStateUpdate { old, new } => {
            update_inout(ctx, data, old, new).await;
        }
        _ => {}
    }
    Ok(())
}
