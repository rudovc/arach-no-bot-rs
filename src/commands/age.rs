use crate::types;

use poise::serenity_prelude as serenity;

// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: types::PoiseContext<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> anyhow::Result<()> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}