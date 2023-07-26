use dotenv::dotenv;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use std::env;

use crate::{
    permissions::{permissions_check, Permissions, PermissionsError},
    state::AuthorizedServerUsers,
};

#[derive(Debug)]
/// A user that is logged in to the website, but not necessarily on the server.
pub struct BasicUser {
    pub discord_token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicUser {
    type Error = PermissionsError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = match req.headers().get_one("Authorization") {
            Some(token) => token.replace("Bearer ", ""),
            None => {
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    PermissionsError::MissingToken,
                ))
            }
        };

        // This is only used for a "Logged in as..." message on the website.
        // We do not need to check if the token is actually valid.
        Outcome::Success(BasicUser {
            discord_token: token,
        })
    }
}

/// A user that is on the discord server.
/// This is used for checking the GET endpoints.
pub struct ServerUser {
    pub discord_token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServerUser {
    type Error = PermissionsError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = match req.headers().get_one("Authorization") {
            Some(token) => token.replace("Bearer ", ""),
            None => {
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    PermissionsError::MissingToken,
                ))
            }
        };

        // We try to find the token in the cache first.
        let auth_users = req.rocket().state::<AuthorizedServerUsers>();

        if let Some(auth_users) = auth_users {
            let auth_users_hash_map = auth_users.logged_in_users.lock().await;

            let invalid_token = "Invalid token.".to_string();

            let user_id = auth_users_hash_map.get(&token).unwrap_or(&invalid_token);

            let auth_users_hash_set = auth_users.guild_users.lock().await;

            if auth_users_hash_set.contains(user_id) {
                return Outcome::Success(ServerUser {
                    discord_token: token,
                });
            }
        }

        // If the token is not in the cache, we check if the user is on the server.
        let on_server = permissions_check(
            &token,
            &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
            true,
        )
        .await;

        match on_server {
            Ok(Permissions::Admin) | Ok(Permissions::User) => Outcome::Success(ServerUser {
                discord_token: token,
            }),
            Ok(Permissions::None) => {
                if req.route().is_some_and(|route| route.uri == "/api/me") {
                    return Outcome::Forward(());
                }

                Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    PermissionsError::NotOnServer,
                ))
            }
            Err(e) => {
                if req.route().is_some_and(|route| route.uri == "/api/me") {
                    return Outcome::Forward(());
                }

                Outcome::Failure((rocket::http::Status::Unauthorized, e))
            }
        }
    }
}

/// A user that is on the discord server and has administator permissions.
/// Used for verifying the macro_new and macro_delete POST endpoints.
pub struct AdminUser {
    pub discord_token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = PermissionsError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = match req.headers().get_one("Authorization") {
            Some(token) => token.replace("Bearer ", ""),
            None => {
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    PermissionsError::MissingToken,
                ))
            }
        };

        let on_server = permissions_check(
            &token,
            &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
            // For the admin check, we do not want to cache the result.
            true,
        )
        .await;

        match on_server {
            Ok(Permissions::Admin) => Outcome::Success(AdminUser {
                discord_token: token,
            }),
            Ok(Permissions::User) | Ok(Permissions::None) => Outcome::Failure((
                rocket::http::Status::Unauthorized,
                PermissionsError::NotAdmin,
            )),
            Err(e) => Outcome::Failure((rocket::http::Status::Unauthorized, e)),
        }
    }
}
