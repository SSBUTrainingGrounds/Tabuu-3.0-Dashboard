#![allow(non_snake_case)] // False positive for unused variable names.

mod auth;
mod catcher;
mod emoji;
mod hwinfo;
mod level;
mod permissions;
mod rating;
mod requests;
mod routes;
mod state;
mod types;

use dotenv::dotenv;
use std::env;

use rocket::{
    figment::util::map,
    figment::value::{Map, Value},
    futures::lock::Mutex,
    Config,
};
use rocket_sync_db_pools::database;
use state::AuthorizedServerUsers;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use catcher::{bad_request, default, forbidden, not_found, unauthorized};
use routes::{
    commands, get_user, hw_info, index, is_admin, is_on_server, leaderboard, macro_delete,
    macro_get, macro_new, matches, me, me_not_on_guild, profiles, trueskill, users,
};

#[macro_use]
extern crate rocket;

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db: Map<_, Value> = map! {
        "url" => env::var("ROCKET_DATABASE_URL").expect("Invalid ROCKET_DATABASE_URL environment variable").into(),
        "pool_size" => 20.into(),
        "timeout" => 5.into(),
    };

    let address = env::var("ROCKET_ADDRESS").expect("Invalid ROCKET_ADDRESS environment variable");
    let port: u16 = env::var("ROCKET_PORT")
        .expect("Invalid ROCKET_PORT environment variable")
        .parse()
        .expect("Invalid ROCKET_PORT environment variable");

    let figment = Config::figment()
        .merge(("databases", map!["sqlite_database" => db]))
        .merge(("port", port))
        .merge(("address", address));

    rocket::custom(figment)
        .attach(DbConn::fairing())
        .mount(
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
                me,
                me_not_on_guild,
                is_admin,
                is_on_server,
                hw_info
            ],
        )
        .register(
            "/api/",
            catchers![not_found, unauthorized, forbidden, bad_request, default],
        )
        .manage(AuthorizedServerUsers {
            logged_in_users: Arc::new(Mutex::new(HashMap::new())),
            guild_users: Arc::new(Mutex::new(HashSet::new())),
        })
}
