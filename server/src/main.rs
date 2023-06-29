mod types;
mod utils;

use dotenv::dotenv;
use rusqlite::Connection;
use std::env;
use utils::get_users;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hi! Listening on port 8080. Available endpoints: /trueskill, /leaderboard, /commands, /profiles, /macro_get, /macro_new, /macro_delete, /users"
}

#[get("/trueskill")]
fn trueskill() -> String {
    dotenv().ok();

    let conn = Connection::open(
        env::var("DATABASE_LOCATION")
            .expect("You have not set the DATABASE_LOCATION environment variable"),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM trueskill").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(types::TrueSkill {
            // TODO: The user_ids may need to be converted to strings for JavaScript.
            user_id: row.get(0)?,
            rating: row.get(1)?,
            deviation: row.get(2)?,
            wins: row.get(3)?,
            losses: row.get(4)?,
            matches: row.get(5)?,
        })
    });

    let mut trueskill = vec![];

    for user in user_iter.unwrap() {
        trueskill.push(user.unwrap());
    }

    serde_json::to_string(&trueskill).unwrap()
}

#[get("/leaderboard")]
fn leaderboard() -> String {
    dotenv().ok();

    let conn = Connection::open(
        env::var("DATABASE_LOCATION")
            .expect("You have not set the DATABASE_LOCATION environment variable"),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM level").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(types::Leaderboard {
            id: row.get(0)?,
            level: row.get(1)?,
            xp: row.get(2)?,
            messages: row.get(3)?,
        })
    });

    let mut level = vec![];

    for user in user_iter.unwrap() {
        level.push(user.unwrap());
    }

    serde_json::to_string(&level).unwrap()
}

#[get("/commands")]
fn commands() -> String {
    dotenv().ok();

    let conn = Connection::open(
        env::var("DATABASE_LOCATION")
            .expect("You have not set the DATABASE_LOCATION environment variable"),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM commands").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(types::Commands {
            command: row.get(0)?,
            uses: row.get(1)?,
            last_used: row.get(2)?,
        })
    });

    let mut commands = vec![];

    for user in user_iter.unwrap() {
        commands.push(user.unwrap());
    }

    serde_json::to_string(&commands).unwrap()
}

#[get("/profiles")]
fn profiles() -> String {
    dotenv().ok();

    let conn = Connection::open(
        env::var("DATABASE_LOCATION")
            .expect("You have not set the DATABASE_LOCATION environment variable"),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM profile").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(types::Profiles {
            user_id: row.get(0)?,
            tag: row.get(1)?,
            region: row.get(2)?,
            mains: row.get(3)?,
            secondaries: row.get(4)?,
            pockets: row.get(5)?,
            note: row.get(6)?,
            colour: row.get(7)?,
        })
    });

    let mut profiles = vec![];

    for user in user_iter.unwrap() {
        profiles.push(user.unwrap());
    }

    serde_json::to_string(&profiles).unwrap()
}

#[get("/macro_get")]
fn macro_get() -> String {
    dotenv().ok();

    let conn = Connection::open(
        env::var("DATABASE_LOCATION")
            .expect("You have not set the DATABASE_LOCATION environment variable"),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT * FROM macros").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(types::Macros {
            name: row.get(0)?,
            payload: row.get(1)?,
            uses: row.get(2)?,
            author: row.get(3)?,
        })
    });

    let mut macros = vec![];

    for user in user_iter.unwrap() {
        macros.push(user.unwrap());
    }

    serde_json::to_string(&macros).unwrap()
}

#[post("/macro_new")]
fn macro_new() {
    if !utils::admin_check("token", "guild_id") {
        return;
    }

    let conn = Connection::open("./database.db").unwrap();

    // ...
}

#[post("/macro_delete")]
fn macro_delete() {
    if !utils::admin_check("token", "guild_id") {
        return;
    }

    let conn = Connection::open("./database.db").unwrap();

    // ...
}

#[get("/users")]
async fn users() -> String {
    dotenv().ok();

    let users: Vec<types::RawGuildUser> = get_users(
        &env::var("DISCORD_TOKEN")
            .expect("You have not set the DISCORD_TOKEN environment variable"),
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    )
    .await;

    serde_json::to_string(&users).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            index,
            trueskill,
            leaderboard,
            commands,
            profiles,
            macro_get,
            macro_new,
            macro_delete,
            users
        ],
    )
}
