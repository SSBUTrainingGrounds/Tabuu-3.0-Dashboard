use dotenv::dotenv;
use rocket::{serde::json::Json, State};
use std::env;

use crate::{
    auth::{AdminUser, BasicUser, ServerUser},
    emoji::get_emojis_from_str,
    hwinfo::get_hw_info,
    level::{get_level_progress, get_next_role_progress, get_xp_for_level},
    rating::{
        get_average_opponent, get_display_rating, get_recent_matches, get_recent_performance,
        get_streaks, get_last_ratings,
    },
    requests::{fetch_me, fetch_single_user, get_json_string, get_users},
    state::AuthorizedServerUsers,
    types, DbConn,
};

#[get("/")]
pub fn index() -> &'static str {
    "Hi! Available endpoints: 
    GET: /trueskill, /matches, /leaderboard, /commands, /profiles, /macro_get, /users, /user/<user_id>, /me, /hwinfo, /is_admin, /is_on_server
    POST: /macro_new, /macro_delete"
}

#[get("/trueskill")]
pub async fn trueskill(conn: DbConn, _user: ServerUser) -> String {
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
                    let user_id: String = row.get(0)?;
                    let rating: f64 = row.get(1)?;
                    let deviation: f64 = row.get(2)?;
                    let wins: usize = row.get(3)?;
                    let losses: usize = row.get(4)?;
                    let matches: String = row.get(5)?;
                    let streaks: (usize, usize, usize, usize) = get_streaks(matches.clone());

                    let mut recent_matches_stmt = match c.prepare(
                        "SELECT CAST(match_id AS TEXT) AS match_id, CAST(winner_id AS TEXT) AS winner_id,
                        CAST(loser_id AS TEXT) AS loser_id, timestamp, old_winner_rating, 
                        old_winner_deviation, old_loser_rating, old_loser_deviation, new_winner_rating, 
                        new_winner_deviation, new_loser_rating, new_loser_deviation FROM matches 
                        WHERE winner_id = ?1 OR loser_id = ?1 ORDER BY timestamp DESC LIMIT 10"
                    ) {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    };

                    let recent_matches: Vec<types::Matches> = match recent_matches_stmt.query_map([user_id.clone()], {
                        |row| {
                            Ok(types::Matches {
                                match_id: row.get(0)?,
                                winner_id: row.get(1)?,
                                loser_id: row.get(2)?,
                                timestamp: row.get(3)?,
                                old_winner_rating: row.get(4)?,
                                old_winner_deviation: row.get(5)?,
                                old_loser_rating: row.get(6)?,
                                old_loser_deviation: row.get(7)?,
                                new_winner_rating: row.get(8)?,
                                new_winner_deviation: row.get(9)?,
                                new_loser_rating: row.get(10)?,
                                new_loser_deviation: row.get(11)?,
                                old_winner_display_rating: 0.0,
                                old_loser_display_rating: 0.0,
                                new_winner_display_rating: 0.0,
                                new_loser_display_rating: 0.0,
                                winner_display_rating_change: 0.0,
                                loser_display_rating_change: 0.0,
                            })
                        }
                    }) {
                        Ok(matches) => matches,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    }.map(|m| m.unwrap_or(types::Matches {
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
                    })).collect::<Vec<types::Matches>>();

                    let mut best_win_stmt = match c.prepare(
                        "SELECT CAST(match_id AS TEXT) AS match_id, CAST(winner_id AS TEXT) AS winner_id,
                        CAST(loser_id AS TEXT) AS loser_id, timestamp, old_winner_rating, 
                        old_winner_deviation, old_loser_rating, old_loser_deviation, new_winner_rating, 
                        new_winner_deviation, new_loser_rating, new_loser_deviation FROM matches WHERE winner_id = ?1 ORDER BY (old_loser_rating - 3 * old_loser_deviation) DESC LIMIT 1"
                    ) {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    };

                    // This is only a single row, so we can just query the first row and get the result directly.
                    let best_win: types::Matches = best_win_stmt.query_row([user_id.clone()], |row| {
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

                        Ok(types::Matches {
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
                        })
                    }).unwrap_or(types::Matches {
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
                    });

                    let mut average_opponent_stmt = match c.prepare(
                        "SELECT
                            CASE WHEN winner_id = ?1 THEN
                                old_loser_rating
                            ELSE
                                old_winner_rating
                            END rating,
                            CASE WHEN winner_id = ?1 THEN
                                old_loser_deviation
                            ELSE
                                old_winner_deviation
                            END deviation
                        FROM matches
                        WHERE winner_id = ?1 OR loser_id = ?1"
                    ) {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    };

                    let opponents_ratings: Vec<f64> = match average_opponent_stmt.query_map([user_id.clone()], {
                        |row| {
                            let rating: f64 = row.get(0)?;
                            let deviation: f64 = row.get(1)?;

                            let display_rating = get_display_rating(rating, deviation);

                            Ok(display_rating)
                        }
                    }) {
                        Ok(ratings) => ratings,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    }.map(|r| r.unwrap_or(0.0)).collect();

                    let mut highest_rating_stmt = match c.prepare(
                        "
                        SELECT
                            CASE WHEN winner_id = ?1 THEN
                                new_winner_rating
                            ELSE
                                CASE WHEN new_loser_rating - 3 * new_loser_deviation < old_loser_rating - 3 * old_loser_deviation THEN
                                    old_loser_rating
                                ELSE
                                    new_loser_rating
                                END
                            END rating,
                            CASE WHEN winner_id = ?1 THEN
                                new_winner_deviation
                            ELSE
                                CASE WHEN new_loser_rating - 3 * new_loser_deviation < old_loser_rating - 3 * old_loser_deviation THEN
                                    old_loser_deviation
                                ELSE
                                    new_loser_deviation
                                END
                            END deviation,
                            timestamp
                        FROM matches
                        WHERE winner_id = ?1 OR loser_id = ?1
                        ORDER BY rating - 3 * deviation
                        DESC LIMIT 1
                        "
                    ) {
                        Ok(stmt) => stmt,
                        Err(e) => {
                            println!("Error: {}", e);
                            return Err(e);
                        }
                    };

                    // This might be null since we did not record all matches.
                    // In that case, we just use the current rating.
                    let highest_rating: f64 = highest_rating_stmt.query_row([user_id.clone()], |row| {
                        let r: f64 = row.get(0)?;
                        let d: f64 = row.get(1)?;

                        let display_rating = get_display_rating(r, d);

                        Ok(display_rating)
                    }).unwrap_or(get_display_rating(rating, deviation));

                    let last_match = match recent_matches.first() {
                        Some(m) => m.clone(),
                        None => types::Matches {
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
                    };

                    types::TrueSkill {
                        rank: 0,
                        user_id: user_id.clone(),
                        rating,
                        deviation,
                        display_rating: get_display_rating(rating, deviation),
                        wins,
                        losses,
                        matches: matches.clone(),
                        recent_matches: get_recent_matches(matches, 10),
                        win_percentage: (wins as f64 / (wins + losses) as f64) * 100.0,
                        longest_win_streak: streaks.0,
                        longest_loss_streak: streaks.1,
                        current_win_streak: streaks.2,
                        current_loss_streak: streaks.3,
                        all_time_highest_rating: highest_rating,
                        recent_performance: get_recent_performance(&recent_matches, &user_id, rating, deviation),
                        last_ratings: get_last_ratings(&recent_matches, &user_id),
                        avg_opponent_rating: get_average_opponent(&opponents_ratings),
                        highest_win: best_win,
                        last_match,
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
                            recent_matches: String::from(""),
                            win_percentage: 0.0,
                            longest_win_streak: 0,
                            longest_loss_streak: 0,
                            current_win_streak: 0,
                            current_loss_streak: 0,
                            all_time_highest_rating: 0.0,
                            recent_performance: 0.0,
                            last_ratings: vec![],
                            avg_opponent_rating: 0.0,
                            highest_win: types::Matches {
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
                            last_match: types::Matches {
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
pub async fn matches(conn: DbConn, _user: ServerUser) -> String {
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
pub async fn leaderboard(conn: DbConn, _user: ServerUser) -> String {
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
                let id: String = row.get(0)?;
                let level = row.get(1)?;
                let xp = row.get(2)?;

                let (next_role, next_role_progress, xp_to_next_role) = get_next_role_progress(level, xp);

                let mut badge_stmt = match c.prepare(
                    "SELECT badges FROM userbadges WHERE user_id = ?1",
                ) {
                    Ok(stmt) => stmt,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                };

                let badges: Vec<String> = match badge_stmt.query_row([id.clone()], |row| {
                    let badges: String = row.get(0)?;

                    Ok(get_emojis_from_str(badges))
                }) {
                    Ok(badges) => badges,
                    Err(_) => Vec::new(),
                };

                let mut username_stmt = match c.prepare(
                    "SELECT old_name FROM usernames WHERE user_id = ?1 ORDER BY timestamp DESC LIMIT 5",
                ) {
                    Ok(stmt) => stmt,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                };

                let last_five_usernames = match username_stmt.query_map([id.clone()], |row| {
                    row.get(0)
                }) {
                    Ok(usernames) => usernames,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                }.map(|u| u.unwrap_or(String::from(""))).collect();

                let mut nickname_stmt = match c.prepare(
                    "SELECT old_name FROM nicknames WHERE user_id = ?1 ORDER BY timestamp DESC LIMIT 5",
                ) {
                    Ok(stmt) => stmt,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                };

                let last_five_nicknames = match nickname_stmt.query_map([id], |row| {
                    row.get(0)
                }) {
                    Ok(nicknames) => nicknames,
                    Err(e) => {
                        println!("Error: {}", e);
                        return Err(e);
                    }
                }.map(|n| n.unwrap_or(String::from(""))).collect();

                types::Leaderboard {
                    rank: 0,
                    id: row.get(0)?,
                    level,
                    xp,
                    messages: row.get(3)?,
                    xp_progress: get_level_progress(level, xp),
                    xp_to_next_level: get_xp_for_level(level + 1) - xp,
                    next_role,
                    next_role_progress,
                    xp_to_next_role,
                    badges,
                    last_five_usernames,
                    last_five_nicknames,
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
                    xp_to_next_level: 0,
                    next_role: None,
                    next_role_progress: None,
                    xp_to_next_role: None,
                    badges: Vec::new(),
                    last_five_usernames: Vec::new(),
                    last_five_nicknames: Vec::new(),
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
pub async fn commands(conn: DbConn, _user: ServerUser) -> String {
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
pub async fn profiles(conn: DbConn, _user: ServerUser) -> String {
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
                let mains = get_emojis_from_str(row.get(3)?);
                let secondaries = get_emojis_from_str(row.get(4)?);
                let pockets = get_emojis_from_str(row.get(5)?);

                Ok(types::Profiles {
                    user_id: row.get(0)?,
                    tag: row.get(1)?,
                    region: row.get(2)?,
                    mains,
                    secondaries,
                    pockets,
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
                            mains: Vec::new(),
                            secondaries: Vec::new(),
                            pockets: Vec::new(),
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
pub async fn macro_get(conn: DbConn, _user: ServerUser) -> String {
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
            let mut display_payload: String = row.get(1)?;

            if display_payload.len() > 200 {
                display_payload = display_payload[..200].to_string();
                display_payload = format!("{}...", display_payload);
            }

            Ok(types::Macros {
                name: row.get(0)?,
                payload: row.get(1)?,
                display_payload,
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
                    display_payload: String::from(""),
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
pub async fn macro_new(conn: DbConn, input: Json<types::MacroNew>, _user: AdminUser) {
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
pub async fn macro_delete(conn: DbConn, input: Json<types::MacroDelete>, _user: AdminUser) {
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

#[get("/me", rank = 1)]
pub async fn me(user: ServerUser, state: &State<AuthorizedServerUsers>) -> String {
    // This endpoint is called when the page is loaded.
    let logged_in_user = fetch_me(&user.discord_token).await;

    // If the user is on the server, we add them to the authorized logged in users cache.
    if let Some(u) = logged_in_user.clone() {
        let mut authorized_server_users = state.logged_in_users.lock().await;

        authorized_server_users.insert(user.discord_token, u.id);
    }

    get_json_string(logged_in_user)
}

#[get("/me", rank = 2)]
pub async fn me_not_on_guild(user: BasicUser, state: &State<AuthorizedServerUsers>) -> String {
    let user_not_on_guild = fetch_me(&user.discord_token).await;

    // This route only gets fired if the user is not on the server.
    // If the user is not on the server, we remove them from the authorized logged in users cache.
    if user_not_on_guild.is_some() {
        let mut authorized_server_users = state.logged_in_users.lock().await;

        authorized_server_users.remove(&user.discord_token);
    }

    get_json_string(user_not_on_guild)
}

#[get("/users")]
pub async fn users(_user: ServerUser, state: &State<AuthorizedServerUsers>) -> String {
    dotenv().ok();

    let users: Vec<types::RawGuildUser> = get_users(
        &env::var("DISCORD_TOKEN")
            .expect("You have not set the DISCORD_TOKEN environment variable"),
        &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
    )
    .await;

    // Updates the authorized users cache periodically.
    let mut authorized_server_users = state.guild_users.lock().await;

    authorized_server_users.clear();

    for user in &users {
        authorized_server_users.insert(user.user.id.clone());
    }

    // This removes users that are no longer on the server from the authorized logged in users cache.
    let mut authorized_logged_in_users = state.logged_in_users.lock().await;
    let mut users_to_remove: Vec<String> = Vec::new();

    for (token, user_id) in &*authorized_logged_in_users {
        if !authorized_server_users.contains(user_id) {
            users_to_remove.push(token.clone());
            println!(
                "Removing {} from the authorized logged in users cache.",
                user_id
            );
        }
    }

    for token in users_to_remove {
        authorized_logged_in_users.remove(&token);
    }

    get_json_string(users)
}

#[get("/user/<user_id>")]
pub async fn get_user(user_id: &str, _user: ServerUser) -> String {
    dotenv().ok();

    let user = fetch_single_user(
        &env::var("DISCORD_TOKEN")
            .expect("You have not set the DISCORD_TOKEN environment variable"),
        user_id,
    )
    .await;

    get_json_string(user)
}

#[get("/is_admin")]
pub async fn is_admin(_user: AdminUser) -> &'static str {
    "True"
}

#[get("/is_on_server")]
pub async fn is_on_server(_user: ServerUser) -> &'static str {
    "True"
}

#[get("/hwinfo")]
pub async fn hw_info(_user: ServerUser) -> String {
    let hw_info = get_hw_info();

    get_json_string(hw_info)
}

#[cfg(test)]
mod tests {
    use crate::rocket;
    use ::rocket::http::{self, Status};
    use ::rocket::local::blocking::Client;

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
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_matches() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/matches").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_leaderboard() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/leaderboard").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_commands() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/commands").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_profiles() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/profiles").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_macro_get() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/macro_get").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_users() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/users").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_get_user() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/123456789").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_is_admin() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/is_admin").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macro_new() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(r#"{"name": "test", "payload": "test", "uses": 0, "author": "test"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macro_delete() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_delete")
            .body(r#"{"name": "test"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_macros_bad_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/macro_delete").dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(r#"{"payload": "test", "uses": 0, "author": "test"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .post("/api/macro_new")
            .body(r#"{"name": "test", "uses": 0, "author": "test"}"#)
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_is_admin_bad_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/is_admin").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_is_on_server_bad_requests() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post("/api/is_on_server").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_get_user_bad_request() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/user/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_rocket_hw_info() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/hwinfo/").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_me() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/api/me").dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }

    #[test]
    fn test_rocket_me_not_on_guild() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/api/me")
            .header(http::Header::new(
                "Authorization",
                "Bearer Any_Made_Up_Discord_Token",
            ))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
