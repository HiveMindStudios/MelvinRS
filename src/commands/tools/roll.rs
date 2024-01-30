// #![cfg(target_os = "windows")]

use crate::{Context, Error};
use fancy_regex::Regex;
use rand::distributions::{Distribution, Uniform};

#[poise::command(prefix_command, slash_command)]
pub async fn roll(
    ctx: Context<'_>,
    #[description = "Roll parameters"] roll_data: Option<String>,
) -> Result<(), Error> {
    let mut res: i32 = 0;
    let _random = {
        let mut rng = rand::thread_rng();
        let dice_rolls = Regex::new(r"(\+|-|\/|\*)?([\d]*)d([\d]*)(kh|kl|r)?([\d])*").unwrap();
        // let modifiers = Regex::new(r"(\+|-)+([\d]*)(?!d)").unwrap();

        if let Some(roll_data) = roll_data {
            let roll_after_regex = dice_rolls
                .captures(&roll_data)
                .unwrap()
                .expect("Error running regex");
            // let mod_after_regex = modifiers.captures("").unwrap().expect("Error running regex");

            let roll_amount: i32 = roll_after_regex
                .get(2)
                .expect("No group")
                .as_str()
                .parse()
                .unwrap();
            let side_amount: i32 = roll_after_regex
                .get(3)
                .expect("No group")
                .as_str()
                .parse()
                .unwrap();
            let roll_mods = roll_after_regex.get(4).expect("No group").as_str();
            let roll_mods_amount: i32 = roll_after_regex
                .get(5)
                .expect("No group")
                .as_str()
                .parse()
                .unwrap();
            let mut rolls: Vec<i32> = vec![];
            let r = Uniform::from(1..side_amount + 1);
            rolls.sort();
            for _i in 1..roll_amount {
                let throw = r.sample(&mut rng);
                rolls.push(throw);
            }
            if roll_mods == "kh" {
                for _i in 1..roll_mods_amount {
                    let max = rolls.iter().max();
                    let max_index = rolls.iter().position(|x| *x == *max.unwrap()).unwrap();
                    res += rolls[max_index];
                    rolls.remove(max_index);
                }
            } else if roll_mods == "kl" {
                for _i in 1..roll_mods_amount {
                    let min = rolls.iter().min();
                    let min_index = rolls.iter().position(|x| *x == *min.unwrap()).unwrap();
                    res += rolls[min_index];
                    rolls.remove(min_index);
                }
            } else {
                res += rolls.iter().sum::<i32>();
            }
        }
    };
    let response: String = format!("{}", res);
    ctx.say(response).await?;
    Ok(())
}
