use crate::{Context, Error};
use std::time::SystemTime;

#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let now = SystemTime::now();
    let time: i64 = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;
    let msg_time = ctx.created_at().timestamp_millis();

    let response: String = format!("Pong! (latency: {} ms)", time - msg_time);
    ctx.say(response).await?;
    Ok(())
}