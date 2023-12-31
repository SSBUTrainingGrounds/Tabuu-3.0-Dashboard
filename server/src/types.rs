use rocket::serde::{Deserialize, Serialize};

/// A struct for the POST endpoint /macro_new
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroNew {
    pub name: String,
    pub payload: String,
    pub uses: usize,
    pub author: String,
}

/// A struct for the POST endpoint /macro_delete
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MacroDelete {
    pub name: String,
}

/// A struct for the GET endpoint /trueskill
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrueSkill {
    pub rank: usize,
    pub user_id: String,
    pub rating: f64,
    pub deviation: f64,
    pub display_rating: f64,
    pub wins: usize,
    pub losses: usize,
    pub matches: String,
    pub recent_matches: String,
    pub win_percentage: f64,
    pub longest_win_streak: usize,
    pub longest_loss_streak: usize,
    pub current_win_streak: usize,
    pub current_loss_streak: usize,
    // Rating + Timestamp
    pub all_time_highest_rating: (f64, usize),
    pub recent_performance: f64,
    pub last_ratings: Vec<f64>,
    pub avg_opponent_rating: f64,
    pub highest_win: Matches,
    pub last_match: Matches,
}

/// A struct for the GET endpoint /leaderboard
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leaderboard {
    pub rank: usize,
    pub id: String,
    pub level: usize,
    pub xp: usize,
    pub messages: usize,
    pub xp_progress: f64,
    pub xp_to_next_level: usize,
    pub next_role: Option<String>,
    pub next_role_progress: Option<f64>,
    pub xp_to_next_role: Option<usize>,
    pub badges: Vec<String>,
    pub last_five_usernames: Vec<String>,
    pub last_five_nicknames: Vec<String>,
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
    pub mains: Vec<String>,
    pub secondaries: Vec<String>,
    pub pockets: Vec<String>,
    pub note: String,
    pub colour: usize,
}

/// A struct for the GET endpoint /macro_get
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Macros {
    pub name: String,
    pub payload: String,
    pub display_payload: String,
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
    pub banner_color: Option<String>,
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

/// A struct for the POST endpoint /is_admin
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAdmin {
    pub is_admin: bool,
}

/// A struct for the data required for the POST endpoint /is_admin
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsAdminData {
    pub discord_token: String,
}

/// A struct for the data required for the POST endpoint /is_on_server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IsOnServerData {
    pub discord_token: String,
}

/// A struct for the GET endpoint /hwinfo
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HwInfo {
    pub uptime: u64,
    pub os_name: String,
    pub cpu_name: String,
    pub cpu_freq: Vec<u64>,
    pub avg_cpu_freq: u64,
    pub cpu_usage: Vec<f32>,
    pub avg_cpu_usage: f32,
    pub cpu_temp_c: Vec<f32>,
    pub avg_cpu_temp_c: f32,
    pub cpu_temp_f: Vec<f32>,
    pub avg_cpu_temp_f: f32,
    /// Physical and logical cores
    pub cpu_cores: (usize, usize),
    pub ram_total: u64,
    pub ram_used: u64,
    pub ram_free: u64,
    pub ram_percentage: f32,
    pub ram_readable_str: String,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_free: u64,
    pub swap_percentage: f32,
    pub swap_readable_str: String,
    /// Type, Total, Used, Free, Percentage, Readable String
    pub disks: Vec<(String, u64, u64, u64, f32, String)>,
}
