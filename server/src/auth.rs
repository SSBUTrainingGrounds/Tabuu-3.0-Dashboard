use dotenv::dotenv;
use rocket::{
    request::{FromRequest, Outcome},
    Request,
};

use std::env;

use crate::permissions::{permissions_check, Permissions};

/// Checks if the user is on the server.
/// This is used for the GET endpoints.
pub struct ServerUser {}

#[derive(Debug)]
pub enum AuthenticationError {
    InvalidToken,
    MissingToken,
    // Maybe add a rate limit error?
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ServerUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = match req.headers().get_one("Authorization") {
            Some(token) => token.replace("Bearer ", ""),
            None => {
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    AuthenticationError::MissingToken,
                ))
            }
        };

        let on_server = permissions_check(
            &token,
            &env::var("GUILD_ID").expect("You have not set the GUILD_ID environment variable"),
            // Could also be okay to cache this result, not sure.
            true,
        )
        .await;

        // Admins are always allowed
        if on_server != Permissions::None {
            Outcome::Success(ServerUser {})
        } else {
            Outcome::Failure((
                rocket::http::Status::Unauthorized,
                AuthenticationError::InvalidToken,
            ))
        }
    }
}

/// Checks if the user is an admin.
/// Used for verifying the macro_new and macro_delete POST endpoints.
pub struct AdminUser {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AuthenticationError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();

        let token = match req.headers().get_one("Authorization") {
            Some(token) => token.replace("Bearer ", ""),
            None => {
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    AuthenticationError::MissingToken,
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

        if on_server == Permissions::Admin {
            Outcome::Success(AdminUser {})
        } else {
            Outcome::Failure((
                rocket::http::Status::Unauthorized,
                AuthenticationError::InvalidToken,
            ))
        }
    }
}
