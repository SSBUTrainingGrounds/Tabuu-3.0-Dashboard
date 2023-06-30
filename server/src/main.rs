#![allow(clippy::let_unit_value)] // False positive

mod types;
mod utils;

use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use std::env;
use types::TrueSkill;
use utils::{fetch_single_user, get_json_string, get_users};

#[macro_use]
extern crate rocket;

#[database("sqlite_database")]
struct DbConn(rusqlite::Connection);

#[get("/")]
fn index() -> &'static str {
    "Hi! Listening on port 8080.
    Available endpoints: /trueskill, /leaderboard, /commands, /profiles, /macro_get, /macro_new, /macro_delete, /users /user/<user_id>"
}

#[get("/trueskill")]
async fn trueskill(conn: DbConn) -> String {
    conn.run(move |c|
        {
            let mut trueskill = vec![];

            let mut stmt = match c.prepare("SELECT CAST(user_id AS TEXT) AS user_id, rating, deviation, wins, losses, matches FROM trueskill") {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&trueskill);
                }
            };
            let user_iter = match stmt.query_map([], |row| {
                Ok(types::TrueSkill {
                    user_id: row.get(0)?,
                    rating: row.get(1)?,
                    deviation: row.get(2)?,
                    wins: row.get(3)?,
                    losses: row.get(4)?,
                    matches: row.get(5)?,
                })
            }) {
                Ok(user_iter) => user_iter,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&trueskill);
                }
            };


            for user in user_iter {
                trueskill.push(match user {
                    Ok(u) => u,
                    Err(_) => {
                        TrueSkill {
                            user_id: String::from(""),
                            rating: 0.0,
                            deviation: 0.0,
                            wins: 0,
                            losses: 0,
                            matches: String::from(""),
                        }
                    }
                });
            }

            get_json_string(&trueskill)
        }

    ).await
}

#[get("/leaderboard")]
async fn leaderboard(conn: DbConn) -> String {
    conn.run(move |c| {
        let mut leaderboard = vec![];

        let mut stmt =
            match c.prepare("SELECT CAST(id AS TEXT) as id, level, xp, messages FROM level") {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&leaderboard);
                }
            };
        let user_iter = match stmt.query_map([], |row| {
            Ok(types::Leaderboard {
                id: row.get(0)?,
                level: row.get(1)?,
                xp: row.get(2)?,
                messages: row.get(3)?,
            })
        }) {
            Ok(user_iter) => user_iter,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&leaderboard);
            }
        };
        for user in user_iter {
            leaderboard.push(match user {
                Ok(u) => u,
                Err(_) => types::Leaderboard {
                    id: String::from(""),
                    level: 0,
                    xp: 0,
                    messages: 0,
                },
            });
        }

        get_json_string(&leaderboard)
    })
    .await
}

#[get("/commands")]
async fn commands(conn: DbConn) -> String {
    conn.run(move |c| {
        let mut commands = vec![];

        let mut stmt = match c.prepare("SELECT * FROM commands") {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&commands);
            }
        };
        let user_iter = match stmt.query_map([], |row| {
            Ok(types::Commands {
                command: row.get(0)?,
                uses: row.get(1)?,
                last_used: row.get(2)?,
            })
        }) {
            Ok(user_iter) => user_iter,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&commands);
            }
        };
        for user in user_iter {
            commands.push(match user {
                Ok(u) => u,
                Err(_) => types::Commands {
                    command: String::from(""),
                    uses: 0,
                    last_used: 0,
                },
            });
        }

        get_json_string(&commands)
    })
    .await
}

#[get("/profiles")]
async fn profiles(conn: DbConn) -> String {
    conn.run(move |c|
        {
            let mut profiles = vec![];

            let mut stmt = match c.prepare("SELECT CAST(user_id AS TEXT) AS user_id, tag, region, mains, secondaries, pockets, note, colour FROM profile") {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&profiles);
                }
            };
            let user_iter = match stmt.query_map([], |row| {
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
            }) {
                Ok(user_iter) => user_iter,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&profiles);
                }
            };
            for user in user_iter {
                profiles.push(match user {
                    Ok(u) => u,
                    Err(_) => {
                        types::Profiles {
                            user_id: String::from(""),
                            tag: String::from(""),
                            region: String::from(""),
                            mains: String::from(""),
                            secondaries: String::from(""),
                            pockets: String::from(""),
                            note: String::from(""),
                            colour: 0,
                        }
                    }
                });
            }

            get_json_string(&profiles)

        }
    ).await
}

#[get("/macro_get")]
async fn macro_get(conn: DbConn) -> String {
    conn.run(move |c| {
        let mut macros = vec![];

        let mut stmt = match c
            .prepare("SELECT name, payload, uses, CAST(author AS TEXT) as author FROM macros")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&macros);
            }
        };
        let macro_iter = match stmt.query_map([], |row| {
            Ok(types::Macros {
                name: row.get(0)?,
                payload: row.get(1)?,
                uses: row.get(2)?,
                author: row.get(3)?,
            })
        }) {
            Ok(macro_iter) => macro_iter,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&macros);
            }
        };

        for m in macro_iter {
            macros.push(match m {
                Ok(u) => u,
                Err(_) => types::Macros {
                    name: String::from(""),
                    payload: String::from(""),
                    uses: 0,
                    author: String::from(""),
                },
            });
        }

        get_json_string(&macros)
    })
    .await
}

#[post("/macro_new", data = "<input>", format = "application/json")]
async fn macro_new(conn: DbConn, input: Json<types::MacroNew>) {
    let admin = utils::admin_check(
        &input.discord_token,
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    )
    .await;

    if !admin {
        return;
    }

    conn.run(move |c| {
        let mut stmt = match c
            .prepare("INSERT INTO macros (name, payload, uses, author) VALUES (?1, ?2, ?3, ?4)")
        {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        match stmt.execute([
            &input.name,
            &input.payload,
            &input.uses.to_string(),
            &input.author,
        ]) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    })
    .await;
}

#[post("/macro_delete", data = "<input>", format = "application/json")]
async fn macro_delete(conn: DbConn, input: Json<types::MacroDelete>) {
    let admin = utils::admin_check(
        &input.discord_token,
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    )
    .await;

    if !admin {
        return;
    }

    conn.run(move |c| {
        let mut stmt = match c.prepare("DELETE FROM macros WHERE name = ?1") {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        match stmt.execute([&input.name]) {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    })
    .await;
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

    get_json_string(users)
}

#[get("/user/<user_id>")]
async fn get_user(user_id: &str) -> String {
    dotenv().ok();

    let user = fetch_single_user(
        &env::var("DISCORD_TOKEN")
            .expect("You have not set the DISCORD_TOKEN environment variable"),
        user_id,
    )
    .await;

    get_json_string(user)
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
            users,
            get_user
        ],
    )
}
