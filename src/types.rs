pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type PoiseContext<'a> = poise::Context<'a, Data, anyhow::Error>;
