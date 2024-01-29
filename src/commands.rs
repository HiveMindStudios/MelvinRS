use crate::{Context, Error};
use rand::distributions::{Distribution, Uniform};
use regex::Regex;
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

#[poise::command(prefix_command, slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Roll parameters"] roll_data: String,
) -> Result<(), Error> {
    let _random = {
        let mut rng = rand::thread_rng();

        let dice_rolls = Regex::new(r"(\+|-|\/|\*)?([\d]*)d([\d]*)(kh|kl|r)?([\d])*").unwrap();
        // let modifiers = Regex::new(r"(\+|-)+([\d]*)(?!d)").unwrap(); //! this causes an error
        /*
        regex parse error:
        (\+|-)+([\d]*)(?!d)
                      ^^^
        error: look-around, including look-ahead and look-behind, is not supported
        */

        let roll_after_regex = dice_rolls.captures(&roll_data).unwrap();
        // let mod_after_regex = modifiers.captures(&roll_data).unwrap();

        let roll_amount: i32 = roll_after_regex.get(2).unwrap().as_str().parse().unwrap();
        let side_amount: i32 = roll_after_regex.get(3).unwrap().as_str().parse().unwrap();
        // let roll_mods = roll_after_regex.get(4).unwrap().as_str();
        // let roll_mods_amount: i32 = roll_after_regex.get(5).unwrap().as_str().parse().unwrap();
        // let result: i64 = 0;
        let mut rolls: Vec<i32> = vec![];
        let r = Uniform::from(1..side_amount + 1);
        for _i in 1..roll_amount {
            let throw = r.sample(&mut rng);
            rolls.push(throw);
        }
    };
    let response: String = format!("Temp");
    ctx.say(response).await?;
    Ok(())
}
