use crate::{Context, Error};
use std::time::SystemTime;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            // extra_text_at_bottom: "MelvinRS",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

// Templates copied from documentation :)
// #[poise::command(prefix_command, slash_command)]
// pub async fn vote(
//     ctx: Context<'_>,
//     #[description = "What to vote for"] choice: String,
// ) -> Result<(), Error> {
//     let num_votes = {
//         let mut hash_map = ctx.data().votes.lock().unwrap();
//         let num_votes = hash_map.entry(choice.clone()).or_default();
//         *num_votes += 1;
//         *num_votes
//     };

//     let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
//     ctx.say(response).await?;
//     Ok(())
// }

// #[poise::command(prefix_command, track_edits, aliases("votes"), slash_command)]
// pub async fn getvotes(
//     ctx: Context<'_>,
//     #[description = "Choice to retrieve votes for"] choice: Option<String>,
// ) -> Result<(), Error> {
//     if let Some(choice) = choice {
//         let num_votes = *ctx.data().votes.lock().unwrap().get(&choice).unwrap_or(&0);
//         let response = match num_votes {
//             0 => format!("Nobody has voted for {} yet", choice),
//             _ => format!("{} people have voted for {}", num_votes, choice),
//         };
//         ctx.say(response).await?;
//     } else {
//         let mut response = String::new();
//         for (choice, num_votes) in ctx.data().votes.lock().unwrap().iter() {
//             response += &format!("{}: {} votes", choice, num_votes);
//         }

//         if response.is_empty() {
//             response += "Nobody has voted for anything yet :(";
//         }

//         ctx.say(response).await?;
//     };

//     Ok(())
// }

#[poise::command(prefix_command, slash_command)]
pub async fn ping(
    ctx: Context<'_>
) -> Result<(), Error> {
    
    let now = SystemTime::now();
    let time = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as i64;
    let msg_time = ctx.created_at().timestamp_millis();

    let response: String = format!("Pong! (latency: {} ms)", time - msg_time);
    ctx.say(response).await?;
    Ok(())
}