mod json_types;
mod utils;

use rusqlite::Connection;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hi! Listening on port 8080. Available endpoints: /trueskill, /leaderboard, /commands, /profiles, /macro_get, /macro_new, /macro_delete"
}

#[get("/trueskill")]
fn trueskill() -> String {
    let conn = Connection::open("./database.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM trueskill").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(json_types::TrueSkill {
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
    let conn = Connection::open("./database.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM level").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(json_types::Leaderboard {
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
    let conn = Connection::open("./database.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM commands").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(json_types::Commands {
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
    let conn = Connection::open("./database.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM profile").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(json_types::Profiles {
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
    let conn = Connection::open("./database.db").unwrap();

    let mut stmt = conn.prepare("SELECT * FROM macros").unwrap();

    let user_iter = stmt.query_map([], |row| {
        Ok(json_types::Macros {
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
            macro_delete
        ],
    )
}
