use crate::commands::body::*;

#[poise::command(slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}
