use std::io;

use api::{Schedules, RankedGameMode};
use chrono::Duration;

pub mod api;
pub mod mastodon;
pub mod config;

fn format_duration(d: &chrono::Duration) -> String {
    let mut out = "".into();

    if d.num_hours() > 0 {
        out += format!("{} hours, ", d.num_hours()).as_str();
    }

    out += format!("{} minutes and {} seconds", d.num_minutes() % 60, d.num_seconds() % 60).as_str();

    out
}

fn emote_anarchy(s: &RankedGameMode) -> &str {
    match s {
        RankedGameMode::ClamBlitz => &config::CONFIG.emotes.clam_blitz,
        RankedGameMode::TowerControl => &config::CONFIG.emotes.tower_control,
        RankedGameMode::SplatZones => &config::CONFIG.emotes.splat_zones,
        RankedGameMode::Rainmaker => &config::CONFIG.emotes.rainmaker,
    }
}

fn get_sched_text(scheds: &Schedules) -> String {
    let turfwar_list: String = scheds.turf_war.stages.iter()
        .map(|a| format!(" - {}", a.0.clone())).collect::<Vec<String>>().join("\n");

    let anarchy_series_list = {
    let mut o: String = "".into();
        o += format!("**{}** {}\n", scheds.anarchy_series.mode.get_en_name(), emote_anarchy(&scheds.anarchy_series.mode)).as_str();
        for i in &scheds.anarchy_series.stages {
            o += format!(" - {}\n", i.0.clone()).as_str();
        }
        o
    };

    let anarchy_open_list = {
        let mut o: String = "".into();
        o += format!("**{}** {}\n", scheds.anarchy_open.mode.get_en_name(), emote_anarchy(&scheds.anarchy_open.mode)).as_str();
        for i in &scheds.anarchy_open.stages {
            o += format!(" - {}\n", i.0.clone()).as_str();
        }
        o
    };

    let salmon_run_list = {
        let mut o: String = "".into();
        let sr = &scheds.salmon_run;
        o += format!("**{}**\n", sr.stage.0.clone()).as_str();
        for i in &sr.weapons {
            o += format!(" - {}\n", i.0.clone()).as_str();
        }
        o
    };

    format!("**Multiplayer maps and modes have been updated!**

{} Turf War:
{}

{} Anarchy Battle (Series): {}

{} Anarchy Battle (Open): {}

{} Salmon Run: {}

",
    config::CONFIG.emotes.turf_war,
    turfwar_list,
    config::CONFIG.emotes.ranked,
    anarchy_series_list,
    config::CONFIG.emotes.ranked,
    anarchy_open_list,
    config::CONFIG.emotes.salmon_run,
    salmon_run_list)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mastodon = {
        if let None = config::CONFIG.credentials.auth_code {
            let redir_uri = format!(
                "{}/oauth/authorize?client_id={}&scope=read+write+follow+push&redirect_uri=urn:ietf:wg:oauth:2.0:oob&response_type=code",
                config::CONFIG.credentials.instance_uri,
                config::CONFIG.credentials.client_id);
            println!("Please visit {} in your browser and specify the return value, then push Enter.", redir_uri);
            let mut authcode_buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut authcode_buffer)?;
            authcode_buffer = authcode_buffer.trim().to_string();
            let mut m = mastodon::Mastodon::new(authcode_buffer);
            m.login().await;
            m
        } else {
            mastodon::Mastodon::new_with_auth(
                config::CONFIG.credentials.auth_code.as_ref().unwrap().clone())
        }
    };

    let mut scheds = api::get_latest_schedules().await;

    if std::env::args().collect::<Vec<String>>()[1] == "force" {
        println!("forcing post");
        mastodon.make_post(&get_sched_text(&scheds)).await;
        return Ok(())
    }

    loop {
        let now = chrono::Utc::now();
        let next = &scheds.turf_war.ends_at;
        let dur = next.signed_duration_since(now);
        // just so we pull in up-to-date data...
        let dur = dur + Duration::seconds(60);
        
        println!("posting in {}", format_duration(&dur));
        tokio::time::sleep(dur.to_std().unwrap()).await;

        scheds = api::get_latest_schedules().await;
        mastodon.make_post(&get_sched_text(&scheds)).await;
        println!("post made, updating info");
    }
}
