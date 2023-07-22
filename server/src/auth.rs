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
            false,
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
