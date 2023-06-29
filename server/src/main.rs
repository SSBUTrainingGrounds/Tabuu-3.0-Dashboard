mod utils;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hi! Listening on port 8080. Available endpoints: /trueskill, /leaderboard, /commands, /profiles, /macro_get, /macro_new, /macro_delete"
}

#[get("/trueskill")]
fn trueskill() -> &'static str {
    "trueskill"
}

#[get("/leaderboard")]
fn leaderboard() -> &'static str {
    "leaderboard"
}

#[get("/commands")]
fn commands() -> &'static str {
    "commands"
}

#[get("/profiles")]
fn profiles() -> &'static str {
    "profiles"
}

#[get("/macro_get")]
fn macro_get() -> &'static str {
    "macro_get"
}

#[get("/macro_new")]
fn macro_new() -> &'static str {
    "macro_new"
}

#[get("/macro_delete")]
fn macro_delete() -> &'static str {
    "macro_delete"
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
