#![allow(clippy::let_unit_value)] // False positive

mod hwinfo;
mod level;
mod rating;
mod requests;
mod types;

use dotenv::dotenv;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_sync_db_pools::database;
use std::env;

use level::get_level_progress;
use rating::get_display_rating;
use requests::{fetch_single_user, get_json_string, get_users};

#[macro_use]
extern crate rocket;

#[database("sqlite_database")]
struct DbConn(rusqlite::Connection);

#[get("/")]
fn index() -> &'static str {
    "Hi! Available endpoints: 
    GET: /trueskill, /matches, /leaderboard, /commands, /profiles, /macro_get, /users, /user/<user_id>, /hwinfo
    POST: /macro_new, /macro_delete, /is_admin"
}

#[get("/trueskill")]
async fn trueskill(conn: DbConn) -> String {
    conn.run(move |c|
        {
            let mut trueskill = vec![];

            let mut stmt = match c.prepare(
                "SELECT CAST(user_id AS TEXT) AS user_id, rating, deviation, wins, losses, matches FROM trueskill"
            ) {
                Ok(stmt) => stmt,
                Err(e) => {
                    println!("Error: {}", e);
                    return get_json_string(&trueskill);
                }
            };
            let user_iter = match stmt.query_map([], |row| {
                Ok( {
                    let rating = row.get(1)?;
                    let deviation = row.get(2)?;

                    types::TrueSkill {
                        rank: 0,
                        user_id: row.get(0)?,
                        rating,
                        deviation,
                        display_rating: get_display_rating(rating, deviation),
                        wins: row.get(3)?,
                        losses: row.get(4)?,
                        matches: row.get(5)?,
                    }
                }
                    )
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
                        types::TrueSkill {
                            rank: 0,
                            user_id: String::from(""),
                            rating: 0.0,
                            deviation: 0.0,
                            display_rating: 0.0,
                            wins: 0,
                            losses: 0,
                            matches: String::from(""),
                        }
                    }
                });
            }

            trueskill.sort_by(|a, b| b.display_rating.partial_cmp(&a.display_rating).unwrap_or(std::cmp::Ordering::Equal));

            let mut rank = 1;

            for user in &mut trueskill {
                user.rank = rank;
                rank += 1;
            }

            get_json_string(&trueskill)
        }

    ).await
}

#[get("/matches")]
async fn matches(conn: DbConn) -> String {
    conn.run(move |c| {
        let mut matches = vec![];

        let mut stmt = match c.prepare(
            "SELECT CAST(match_id AS TEXT) AS match_id, CAST(winner_id AS TEXT) AS winner_id,
                CAST(loser_id AS TEXT) AS loser_id, timestamp, old_winner_rating, 
                old_winner_deviation, old_loser_rating, old_loser_deviation, new_winner_rating, 
                new_winner_deviation, new_loser_rating, new_loser_deviation FROM matches",
        ) {
            Ok(stmt) => stmt,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&matches);
            }
        };
        let user_iter = match stmt.query_map([], |row| {
            Ok({
                let old_winner_rating = row.get(4)?;
                let old_winner_deviation = row.get(5)?;
                let old_loser_rating = row.get(6)?;
                let old_loser_deviation = row.get(7)?;
                let new_winner_rating = row.get(8)?;
                let new_winner_deviation = row.get(9)?;
                let new_loser_rating = row.get(10)?;
                let new_loser_deviation = row.get(11)?;

                let old_winner_display_rating =
                    get_display_rating(old_winner_rating, old_winner_deviation);
                let old_loser_display_rating =
                    get_display_rating(old_loser_rating, old_loser_deviation);
                let new_winner_display_rating =
                    get_display_rating(new_winner_rating, new_winner_deviation);
                let new_loser_display_rating =
                    get_display_rating(new_loser_rating, new_loser_deviation);

                types::Matches {
                    match_id: row.get(0)?,
                    winner_id: row.get(1)?,
                    loser_id: row.get(2)?,
                    timestamp: row.get(3)?,
                    old_winner_rating,
                    old_winner_deviation,
                    old_loser_rating,
                    old_loser_deviation,
                    new_winner_rating,
                    new_winner_deviation,
                    new_loser_rating,
                    new_loser_deviation,
                    old_winner_display_rating,
                    old_loser_display_rating,
                    new_winner_display_rating,
                    new_loser_display_rating,
                    winner_display_rating_change: new_winner_display_rating
                        - old_winner_display_rating,
                    loser_display_rating_change: new_loser_display_rating
                        - old_loser_display_rating,
                }
            })
        }) {
            Ok(user_iter) => user_iter,
            Err(e) => {
                println!("Error: {}", e);
                return get_json_string(&matches);
            }
        };
        for user in user_iter {
            matches.push(match user {
                Ok(u) => u,
                Err(_) => types::Matches {
                    match_id: String::from(""),
                    winner_id: String::from(""),
                    loser_id: String::from(""),
                    timestamp: 0,
                    old_winner_rating: 0.0,
                    old_winner_deviation: 0.0,
                    old_loser_rating: 0.0,
                    old_loser_deviation: 0.0,
                    new_winner_rating: 0.0,
                    new_winner_deviation: 0.0,
                    new_loser_rating: 0.0,
                    new_loser_deviation: 0.0,
                    old_winner_display_rating: 0.0,
                    old_loser_display_rating: 0.0,
                    new_winner_display_rating: 0.0,
                    new_loser_display_rating: 0.0,
                    winner_display_rating_change: 0.0,
                    loser_display_rating_change: 0.0,
                },
            });
        }

        get_json_string(&matches)
    })
    .await
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
            Ok({
                let level = row.get(1)?;
                let xp = row.get(2)?;

                types::Leaderboard {
                    rank: 0,
                    id: row.get(0)?,
                    level,
                    xp,
                    messages: row.get(3)?,
                    xp_progress: get_level_progress(level, xp),
                }
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
                    rank: 0,
                    id: String::from(""),
                    level: 0,
                    xp: 0,
                    messages: 0,
                    xp_progress: 0.0,
                },
            });
        }

        leaderboard.sort_by(|a, b| b.xp.cmp(&a.xp));

        let mut rank = 1;
        for user in &mut leaderboard {
            user.rank = rank;
            rank += 1;
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

            let mut stmt = match c.prepare(
                "SELECT CAST(user_id AS TEXT) AS user_id, tag, region, mains, secondaries, pockets, note, colour FROM profile"
            ) {
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
    let admin = requests::admin_check(
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
    let admin = requests::admin_check(
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

#[post("/is_admin", data = "<input>", format = "application/json")]
async fn is_admin(
    input: Json<types::IsAdminData>,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    dotenv().ok();

    let result = requests::admin_check(
        &input.discord_token,
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    )
    .await;

    if result {
        Ok(status::Accepted(Some("True".to_string())))
    } else {
        Err(status::Unauthorized(Some("False".to_string())))
    }
}

#[get("/hwinfo")]
async fn hw_info() -> String {
    let hw_info = hwinfo::get_hw_info();

    get_json_string(hw_info)
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(DbConn::fairing()).mount(
        "/api/",
        routes![
            index,
            trueskill,
            matches,
            leaderboard,
            commands,
            profiles,
            macro_get,
            macro_new,
            macro_delete,
            users,
            get_user,
            is_admin,
            hw_info
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_rocket_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_trueskill() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/trueskill").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_matches() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/matches").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_leaderboard() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/leaderboard").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_commands() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/commands").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_profiles() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/profiles").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_macro_get() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/macro_get").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_users() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/users").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_get_user() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/123456789").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_rocket_is_admin() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/is_admin")
            .body(r#"{"discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macro_new() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(r#"{"name": "test", "payload": "test", "uses": 0, "author": "test", "discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macro_delete() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_delete")
            .body(r#"{"name": "test", "discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macros_bad_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_delete")
            .body(r#"{"discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(
                r#"{"payload": "test", "uses": 0, "author": "test", "discord_token": "123456789"}"#,
            )
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(r#"{"name": "test", "uses": 0, "author": "test", "discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_is_admin_bad_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/is_admin")
            .body(r#"{"discord_token": "123456789"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_get_user_bad_request() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
