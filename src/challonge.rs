use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Match {
    #[serde(rename = "match")]
    pub match_field: MatchInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchInfo {
    pub id: i64,
    pub tournament_id: i64,
    pub state: String,
    pub player1_id: i64,
    pub player2_id: i64,
    pub player1_prereq_match_id: Value,
    pub player2_prereq_match_id: Value,
    pub player1_is_prereq_match_loser: bool,
    pub player2_is_prereq_match_loser: bool,
    pub winner_id: Value,
    pub loser_id: Value,
    pub started_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub identifier: String,
    pub has_attachment: bool,
    pub round: i64,
    pub player1_votes: Value,
    pub player2_votes: Value,
    pub group_id: i64,
    pub attachment_count: Value,
    pub scheduled_time: Value,
    pub location: Value,
    pub underway_at: Value,
    pub optional: Value,
    pub rushb_id: Value,
    pub completed_at: Value,
    pub suggested_play_order: i64,
    pub forfeited: Value,
    pub open_graph_image_file_name: Value,
    pub open_graph_image_content_type: Value,
    pub open_graph_image_file_size: Value,
    pub prerequisite_match_ids_csv: String,
    pub scores_csv: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    pub participant: ParticipantInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub active: bool,
    pub checked_in_at: Value,
    pub created_at: String,
    pub final_rank: Value,
    pub group_id: Value,
    pub icon: Value,
    pub id: i64,
    pub invitation_id: Value,
    pub invite_email: Value,
    pub misc: Value,
    pub name: String,
    pub on_waiting_list: bool,
    pub seed: i64,
    pub tournament_id: i64,
    pub updated_at: String,
    pub challonge_username: Value,
    pub challonge_email_address_verified: Value,
    pub removable: bool,
    pub participatable_or_invitation_attached: bool,
    pub confirm_remove: bool,
    pub invitation_pending: bool,
    pub display_name_with_invitation_email_address: String,
    pub email_hash: Value,
    pub username: Value,
    pub attached_participatable_portrait_url: Value,
    pub can_check_in: bool,
    pub group_player_ids: Vec<i64>,
    pub checked_in: bool,
    pub reactivatable: bool,
}

pub async fn get_matches() -> Vec<Match> {
    let config = Config::parse();
    let client = reqwest::Client::new();
    let tournament = format!("https://api.challonge.com/v1/tournaments/{}/matches.json", config.tournamentid);
    let resp = client
        .get(tournament)
        .basic_auth(config.user, Some(config.secret))
        .send()
        .await
        .unwrap()
        .json::<Vec<Match>>()
        .await
        .unwrap();
    return resp;
}

pub fn get_participant_from_id(id: i64) -> ParticipantInfo {
    let participants = get_particpants();
    let matching_participant = participants
        .into_iter()
        .find(|p| p.participant.group_player_ids[0] == id);
    matching_participant.unwrap().participant
}

pub fn get_particpants() -> Vec<Participant> {
    let config = Config::parse();
    let client = reqwest::blocking::Client::new();
    let tournament = format!("https://api.challonge.com/v1/tournaments/{}/participants.json", config.tournamentid);
    let participants = client
        .get(tournament)
        .basic_auth(config.user, Some(config.secret))
        .send()
        .unwrap()
        .json::<Vec<Participant>>()
        .unwrap();
    return participants;
}
