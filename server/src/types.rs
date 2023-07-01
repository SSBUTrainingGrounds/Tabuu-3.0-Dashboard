use rocket::serde::{Deserialize, Serialize};

/// A struct for the POST endpoint /macro_new
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroNew {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: String,
    pub discord_token: String,
}

/// A struct for the POST endpoint /macro_delete
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroDelete {
    pub discord_token: String,
    pub name: String,
}

/// A struct for the GET endpoint /trueskill
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrueSkill {
    pub user_id: String,
    pub rating: f64,
    pub deviation: f64,
    pub display_rating: f64,
    pub wins: usize,
    pub losses: usize,
    pub matches: String,
}

/// A struct for the GET endpoint /leaderboard
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leaderboard {
    pub id: String,
    pub level: usize,
    pub xp: usize,
    pub messages: usize,
    pub xp_progress: f64,
}

/// A struct for the GET endpoint /commands
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commands {
    pub command: String,
    pub uses: usize,
    pub last_used: usize,
}

/// A struct for the GET endpoint /profiles
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

/// A struct for the GET endpoint /macro_get
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macros {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: String,
}

/// Extra user data for the `RawGuildUser` struct.
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

/// A struct for the GET endpoint /users
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

/// A struct for the GET endpoint /user/<user_id>
/// Different to the `RawGuildUser`, this is what gets returned from the Discord API.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FetchedUser {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<usize>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<usize>,
    pub premium_type: Option<usize>,
    pub public_flags: Option<usize>,
}

/// A struct for the GET endpoint /matches
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Matches {
    pub match_id: String,
    pub winner_id: String,
    pub loser_id: String,
    pub timestamp: usize,
    pub old_winner_rating: f64,
    pub old_winner_deviation: f64,
    pub old_loser_rating: f64,
    pub old_loser_deviation: f64,
    pub new_winner_rating: f64,
    pub new_winner_deviation: f64,
    pub new_loser_rating: f64,
    pub new_loser_deviation: f64,
    pub old_winner_display_rating: f64,
    pub old_loser_display_rating: f64,
    pub new_winner_display_rating: f64,
    pub new_loser_display_rating: f64,
    pub winner_display_rating_change: f64,
    pub loser_display_rating_change: f64,
}

/// A struct for the GET endpoint /is_admin
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAdmin {
    pub is_admin: bool,
}

/// A struct for the data required for the GET endpoint /is_admin
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAdminData {
    pub discord_token: String,
    pub guild_id: String,
}
