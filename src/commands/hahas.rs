use crate::types;
use poise::serenity_prelude as serenity;

// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn hahas(
    ctx: types::PoiseContext<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> anyhow::Result<()> {
    let hahas_count = 420;
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{} has been awarded {} hahas (this doesn't work yet)",
        u.name, hahas_count
    );
    ctx.say(response).await?;
    Ok(())
}
