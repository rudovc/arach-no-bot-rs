use crate::types;
use anyhow::Result;
use poise::serenity_prelude as serenity;

// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn hahas(
    ctx: types::PoiseContext<'_>,
    #[description = "User to get hahas for"] user: Option<serenity::User>,
) -> Result<()> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let hahas = ctx
        .data()
        .connection
        .at("users")
        .at(&user.id.to_string())
        .at("haha_count");

    let result = if let Ok(haha_count) = hahas.get::<u32>().await {
        haha_count
    } else {
        0
    };

    let response = format!("{} has been awarded {} hahas", user.name, result);

    ctx.say(response).await?;
    Ok(())
}
