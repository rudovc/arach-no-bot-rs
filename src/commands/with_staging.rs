use crate::staging;

fn command_with_staging<T, E>(command: poise::Command<T, E>) -> poise::Command<T, E> {
    poise::Command {
        checks: vec![|ctx| {
            let is_allowed_channel = staging::is_allowed_channel_in_current_mode(ctx.channel_id());

            Box::pin(async move { Ok(is_allowed_channel) })
        }],
        ..command
    }
}

pub fn commands_with_staging<T, E>(
    commands: Vec<poise::Command<T, E>>,
) -> Vec<poise::Command<T, E>> {
    commands
        .into_iter()
        .map(|command| command_with_staging(command))
        .collect()
}
