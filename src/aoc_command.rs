use super::aoc_get;
use super::format;
use chrono::{Datelike, Utc};
// use serenity::builder::CreateEmbed;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use std::env;

pub async fn aoc_command(command: &ApplicationCommandInteraction) -> String {
    let current_date = chrono::Utc::now();
    let month = current_date.month();
    let default_year: u16 = match month {
        12 => current_date.year() as u16,
        _ => current_date.year() as u16 - 1,
    };

    let year: u16 = match command
        .data
        .options
        .iter()
        .find(|option| option.name == "year")
    {
        Some(option) => match option.resolved.as_ref().expect("Expected year") {
            ApplicationCommandInteractionDataOptionValue::Integer(year) => *year as u16,
            _ => default_year,
        },
        None => default_year,
    };
    // check if year is 2023
    if year == 2023 {
        let tz = chrono::FixedOffset::east_opt(3600 * -5).unwrap();

        let current_time = chrono::Utc::now();

        let deadline = chrono::NaiveDate::from_ymd_opt(2023, 12, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(tz)
            .unwrap()
            .with_timezone(&Utc);

        let diff = deadline - current_time;

        let seconds = diff.num_seconds() % 60;
        let minutes = (diff.num_seconds() / 60) % 60;
        let hours = (diff.num_seconds() / 60) / 60;

        println!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds);

        if diff.num_seconds() > 0 {
            //embed code idk how to make it work yet
            // let mut embed = CreateEmbed::default()
            //     .title(format!("Advent of Code {} starts in...", year))
            //     .description(format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds));
            // embed
            return format!(
                "**\\🎁 \\🎅 \\🌟 Advent of Code {}  \\☃️ \\🎄 \\❄️**\n```Starts in {:0>2} hours {:0>2} minutes {:0>2} seconds.\nGo back and suffer with 3MI3 for now.```",
                year, hours, minutes, seconds
            );
        }
    }

    let session: String = env::var("AOC_SESSION")
        .expect("Expected a session cookie in the environment")
        .parse()
        .expect("Session is not valid");

    let lb_id: u32 = env::var("AOC_LB")
        .expect("Expected a leaderboard ID in the environment")
        .parse()
        .expect("Leaderboard ID is not valid");

    if let Ok(lb) = aoc_get::fetch_leaderboard(lb_id, year, session).await {
        // limit to 25 members
        let members = lb
            .members
            .iter()
            // .filter(|x| x.local_score > 0)
            .take(25)
            .collect::<Vec<&aoc_get::Member>>();

        let mut longest = members
            .iter()
            .map(|x| {
                if let Some(name) = &x.name {
                    name.len()
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        if longest < 9 {
            longest = 9
        }
        let leaderboard = members
            .iter()
            .enumerate()
            .map(|(i, member)| {
                let mut medal = "";
                if i == 0 {
                    medal = "🥇";
                } else if i == 1 {
                    medal = "🥈";
                } else if i == 2 {
                    medal = "🥉";
                }

                let name: String = member
                    .name
                    .as_ref()
                    .unwrap_or(&"Anonymous".to_string())
                    .to_string();

                format!(
                    "{}) {} - {} {} {} ⭐ {}",
                    format::add_padding((i + 1).to_string(), 2),
                    format::add_padding(member.local_score.to_string(), 4),
                    name,
                    format::get_dashes(longest, name.len()),
                    format::add_padding(member.stars.to_string(), 2),
                    medal
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "**\\🎁 \\🎅 \\🌟 Advent of Code Leaderboard {0} \\☃️ \\🎄 \\❄️**\n*https://adventofcode.com/{0}/leaderboard/private/view/{1}*```{2}```",
            year, lb_id, leaderboard
        )
        //embed code idk how to make it work yet
        // let mut embed = CreateEmbed::default()
        //     .title(format!("Advent of Code Leaderboard {}", year))
        //     .description(format!(
        //         "*https://adventofcode.com/{0}/leaderboard/private/view/{1}*",
        //         year, lb_id
        //     ));
        // for (i, member) in members.iter().enumerate() {
        //     let name = member
        //         .name
        //         .as_ref()
        //         .unwrap_or(&"Anonymous".to_string())
        //         .to_string();
        //     embed.field(name, member.local_score.to_string(), false);
        // }
        // embed
    } else {
        "Not found".to_string()

        //embed code idk how to make it work yet
        // let mut embed = CreateEmbed::default()
        //     .title(format!("Advent of Code Leaderboard {}", year))
        //     .description(format!(
        //         "*https://adventofcode.com/{0}/leaderboard/private/view/{1}*",
        //         year, lb_id
        //     ));
        // embed
    }
}
