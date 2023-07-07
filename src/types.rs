pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;
pub type PoiseContext<'a> = poise::Context<'a, Data, PoiseError>;
