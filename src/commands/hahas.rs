use std::collections::HashMap;

use crate::types::{self, UserRecord};
use color_eyre::{eyre::eyre, Result};
use poise::serenity_prelude::{User, UserId};

// Displays your or another user's account creation date
#[poise::command(slash_command, subcommands("count", "list"))]
pub async fn hahas(
    _ctx: types::PoiseContext<'_>,
    #[description = "User to get hahas for"] _user: Option<User>,
) -> Result<()> {
    Ok(())
}

#[poise::command(slash_command)]
pub async fn list(
    ctx: types::PoiseContext<'_>,
    #[description = "User to highlight in the leaderboard"] user: Option<User>,
) -> Result<()> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let users = ctx
        .data()
        .connection
        .at("users")
        .with_params()
        .order_by("\"haha_count\"")
        .limit_to_first(100)
        .finish()
        .get::<HashMap<UserId, UserRecord>>()
        .await
        .map_err(|err| eyre!(err))?;

    let user_haha_count = ctx
        .data()
        .connection
        .at("users")
        .at(&user.id.to_string())
        .get::<UserRecord>()
        .await
        .map_err(|err| eyre!(err))?
        .haha_count;

    let mut users = users.into_iter();

    let mut top_10: Vec<(UserId, UserRecord)> = vec![];

    users.by_ref().take(10).for_each(|user| {
        top_10.push(user);
    });

    let mut lowest_in_top_10 = top_10
        .iter()
        .min_by_key(|u| u.1.haha_count)
        .unwrap()
        .1
        .haha_count;

    for user in users {
        if user.1.haha_count > lowest_in_top_10 {
            let lowest_index = top_10
                .iter()
                .position(|u| u.1.haha_count == lowest_in_top_10)
                .unwrap();

            top_10.remove(lowest_index);
            top_10.push(user);

            lowest_in_top_10 = top_10
                .iter()
                .min_by_key(|u| u.1.haha_count)
                .unwrap()
                .1
                .haha_count;
        }
    }

    top_10.sort_unstable_by_key(|u| u.1.haha_count);

    let mut user_is_in_top_10 = false;

    let reply = {
        let mut lines: Vec<String> = top_10
            .iter()
            .rev()
            .enumerate()
            .map(|(i, u)| {
                let mut decoration = "";
                if u.0 == user.id {
                    decoration = "**";
                    user_is_in_top_10 = true;
                }

                let decorated_name = format!("{}{}{}", decoration, u.1.name, decoration);

                format!("- {}. {}: {} hahas", i + 1, decorated_name, u.1.haha_count)
            })
            .collect();

        if !user_is_in_top_10 {
            lines.push(format!("...\n**{}**: {} hahas", user.name, user_haha_count));
        }

        format!("Top users by hahas:\n>>> {}", lines.join("\n"))
    };

    ctx.say(reply).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn count(
    ctx: types::PoiseContext<'_>,
    #[description = "User to get hahas for"] user: Option<User>,
) -> Result<()> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let hahas = ctx
        .data()
        .connection
        .at("users")
        .at(&user.id.to_string())
        .at("haha_count");

    let result = hahas.get::<u32>().await.unwrap_or(0);

    let reply = format!("{} has been awarded {} hahas", user.name, result);

    ctx.say(reply).await?;
    Ok(())
}

// TODO: Write tests for this horror show
