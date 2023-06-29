#![allow(clippy::let_unit_value)] // False positive

mod types;
mod utils;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use std::env;
use utils::get_users;

#[macro_use]
extern crate rocket;

#[database("sqlite_database")]
struct DbConn(rusqlite::Connection);

#[get("/")]
fn index() -> &'static str {
    "Hi! Listening on port 8080. Available endpoints: /trueskill, /leaderboard, /commands, /profiles, /macro_get, /macro_new, /macro_delete, /users"
}

#[get("/trueskill")]
async fn trueskill(conn: DbConn) -> String {
    conn.run(move |c| 
        {
            let mut trueskill = vec![];

            let mut stmt = c.prepare("SELECT CAST(user_id AS TEXT) AS user_id, rating, deviation, wins, losses, matches FROM trueskill").unwrap();
            let user_iter = stmt.query_map([], |row| {
                Ok(types::TrueSkill {
                    user_id: row.get(0)?,
                    rating: row.get(1)?,
                    deviation: row.get(2)?,
                    wins: row.get(3)?,
                    losses: row.get(4)?,
                    matches: row.get(5)?,
                })
            });
            for user in user_iter.unwrap() {
                trueskill.push(user.unwrap());
            }

            serde_json::to_string(&trueskill).unwrap()
        }

    
    ).await
}

#[get("/leaderboard")]
async fn leaderboard(conn: DbConn) -> String {
    conn.run(move |c| 
        {
            let mut leaderboard = vec![];

            let mut stmt = c.prepare("SELECT CAST(id AS TEXT) as id, level, xp, messages FROM level").unwrap();
            let user_iter = stmt.query_map([], |row| {
                Ok(types::Leaderboard {
                    id: row.get(0)?,
                    level: row.get(1)?,
                    xp: row.get(2)?,
                    messages: row.get(3)?,
                })
            });
            for user in user_iter.unwrap() {
                leaderboard.push(user.unwrap());
            }

            serde_json::to_string(&leaderboard).unwrap()
        }

    
    ).await
}

#[get("/commands")]
async fn commands(conn: DbConn) -> String {
    conn.run(move |c| 
        {
            let mut commands = vec![];

            let mut stmt = c.prepare("SELECT * FROM commands").unwrap();
            let user_iter = stmt.query_map([], |row| {
                Ok(types::Commands {
                    command: row.get(0)?,
                    uses: row.get(1)?,
                    last_used: row.get(2)?,
                })
            });
            for user in user_iter.unwrap() {
                commands.push(user.unwrap());
            }

            serde_json::to_string(&commands).unwrap()

        }
    ).await
}

#[get("/profiles")]
async fn profiles(conn: DbConn) -> String {
    conn.run(move |c| 
        {
            let mut profiles = vec![];

            let mut stmt = c.prepare("SELECT CAST(user_id AS TEXT) AS user_id, tag, region, mains, secondaries, pockets, note, colour FROM profile").unwrap();
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
            for user in user_iter.unwrap() {
                profiles.push(user.unwrap());
            }

            serde_json::to_string(&profiles).unwrap()

        }
    ).await

    
}

#[get("/macro_get")]
async fn macro_get(conn: DbConn) -> String {
    conn.run(move |c| 
        {
            let mut macros = vec![];

            let mut stmt = c.prepare("SELECT name, payload, uses, CAST(author AS TEXT) as author FROM macros").unwrap();
            let user_iter = stmt.query_map([], |row| {
                Ok(types::Macros {
                    name: row.get(0)?,
                    payload: row.get(1)?,
                    uses: row.get(2)?,
                    author: row.get(3)?,
                })
            });

            for user in user_iter.unwrap() {
                macros.push(user.unwrap());
            }

            serde_json::to_string(&macros).unwrap()

        }

    ).await
}

#[post("/macro_new", data = "<input>", format = "application/json")]
async fn macro_new(conn: DbConn, input: Json<types::MacroNew>) {
    let admin = utils::admin_check(
        &input.discord_token,
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    ).await;

    if !admin {
        return;
    }


    conn.run(move |c| 
        {
            let mut stmt = c.prepare("INSERT INTO macros (name, payload, uses, author) VALUES (?1, ?2, ?3, ?4)").unwrap();
            stmt.execute([&input.name, &input.payload, &input.uses.to_string(), &input.author]).unwrap();
        }
    ).await;
    
}

#[post("/macro_delete", data = "<input>", format = "application/json")]
async fn macro_delete(conn: DbConn, input: Json<types::MacroDelete>) {
    let admin = utils::admin_check(
        &input.discord_token,
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    ).await;

    if !admin {
        return;
    }


    conn.run(move |c| 
        {
            let mut stmt = c.prepare("DELETE FROM macros WHERE name = ?1").unwrap();
            stmt.execute([&input.name]).unwrap();
        }
    ).await;
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
    rocket::build().attach(DbConn::fairing()).mount(
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
