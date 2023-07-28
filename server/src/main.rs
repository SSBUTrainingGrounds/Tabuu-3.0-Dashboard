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

use rocket::futures::lock::Mutex;
use rocket_sync_db_pools::database;
use state::AuthorizedServerUsers;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use routes::{
    commands, get_user, hw_info, index, is_admin, is_on_server, leaderboard, macro_delete,
    macro_get, macro_new, matches, me, me_not_on_guild, profiles, trueskill, users,
};

use catcher::{bad_request, default, forbidden, not_found, unauthorized};

#[macro_use]
extern crate rocket;

#[database("sqlite_database")]
pub struct DbConn(rusqlite::Connection);

#[launch]
fn rocket() -> _ {
    // TODO: Config instead of Rocket.toml
    // https://rocket.rs/v0.5-rc/guide/configuration/

    // TODO: Shields
    // https://api.rocket.rs/v0.5-rc/rocket/shield/index.html

    // TODO: Switch from rusqlite to sqlx (async)
    // https://api.rocket.rs/v0.5-rc/rocket_db_pools/index.html#sqlx-v06

    rocket::build()
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
