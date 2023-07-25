use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use rocket::futures::lock::Mutex;

#[derive(Debug, Clone)]
/// Users that are on the server, so we do not have to make a request to the Discord API for every single request.
pub struct AuthorizedServerUsers {
    pub logged_in_users: Arc<Mutex<HashMap<String, String>>>,
    pub guild_users: Arc<Mutex<HashSet<String>>>,
}
