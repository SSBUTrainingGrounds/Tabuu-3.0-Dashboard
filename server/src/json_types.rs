use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrueSkill {
    pub user_id: usize,
    pub rating: f64,
    pub deviation: f64,
    pub wins: usize,
    pub losses: usize,
    pub matches: String,
}

#[derive(Serialize, Deserialize)]
pub struct Leaderboard {
    pub id: usize,
    pub level: usize,
    pub xp: usize,
    pub messages: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Commands {
    pub command: String,
    pub uses: usize,
    pub last_used: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub user_id: usize,
    pub tag: String,
    pub region: String,
    pub mains: String,
    pub secondaries: String,
    pub pockets: String,
    pub note: String,
    pub colour: usize,
}

#[derive(Serialize, Deserialize)]
pub struct Macros {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: usize,
}
