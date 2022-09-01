use chrono::{DateTime, FixedOffset};
use core::time;
use serde_json::Value::Null;
use serde_json::{Result, Value};
use std::thread;
mod slack;
mod challonge;

#[tokio::main]
async fn main() -> Result<()> {
    poll_loop().await
}

async fn poll_loop() -> Result<()> {
    let mut prev_match_count = 0;
    loop {
        let matches = challonge::get_matches().await;
        let total_match_count = matches.len();
        let completed_matches: Vec<challonge::Match> = matches
            .into_iter()
            .filter(|mtc| mtc.match_field.completed_at != Null)
            .collect();
        let completed_match_count = completed_matches.len();
        if completed_match_count <= prev_match_count || prev_match_count == 0 {
            println!("No new matches.");
            println!("Matches Completed: {}/{}", completed_match_count, total_match_count);
        } else {
            handle_update(completed_matches);
            println!("New match found. Sending message to Slack...");
            println!("Matches Completed: {}/{}", completed_match_count, total_match_count);
        }
        prev_match_count = completed_match_count;
        let sleep_time = time::Duration::from_millis(5000);
        thread::sleep(sleep_time);
    }
}

fn handle_update(matches: Vec<challonge::Match>) {
    thread::spawn(move || {
        let mut mut_matches: Vec<challonge::Match> = matches.clone();
        mut_matches.sort_by(|a, b| {
            parse_date(&a.match_field.completed_at).cmp(&parse_date(&b.match_field.completed_at))
        });
        let latest_match = mut_matches.last().unwrap();
        slack::send_match_msg(latest_match.to_owned());
    });
}

fn parse_date(val: &Value) -> DateTime<FixedOffset> {
    let date_str = val.as_str().unwrap();
    DateTime::parse_from_rfc3339(&date_str)
        .unwrap_or_else(|e| panic!("Date could not parse: {:?}", e))
}
