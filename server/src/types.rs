use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroNew {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: String,
    pub discord_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroDelete {
    pub discord_token: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrueSkill {
    pub user_id: String,
    pub rating: f64,
    pub deviation: f64,
    pub wins: usize,
    pub losses: usize,
    pub matches: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leaderboard {
    pub id: String,
    pub level: usize,
    pub xp: usize,
    pub messages: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commands {
    pub command: String,
    pub uses: usize,
    pub last_used: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profiles {
    pub user_id: String,
    pub tag: String,
    pub region: String,
    pub mains: String,
    pub secondaries: String,
    pub pockets: String,
    pub note: String,
    pub colour: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macros {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawUser {
    pub accent_color: Option<usize>,
    pub avatar: Option<String>,
    pub avatar_decoration: Option<String>,
    pub banner: Option<String>,
    pub banner_color: Option<usize>,
    pub bot: Option<bool>,
    pub discriminator: String,
    pub display_name: Option<String>,
    pub flags: usize,
    pub global_name: Option<String>,
    pub id: String,
    pub public_flags: usize,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawGuildUser {
    pub avatar: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub deaf: bool,
    pub flags: usize,
    pub joined_at: String,
    pub mute: bool,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<String>,
    pub roles: Vec<String>,
    pub user: RawUser,
}
