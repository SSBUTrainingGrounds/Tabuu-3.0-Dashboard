extern crate reqwest;

use async_recursion::async_recursion;
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{header, Client};
use reqwest_middleware::ClientBuilder;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
/// Admin: The user is an admin of the guild.
/// User: The user is a member of the guild.
/// None: The user is not a member of the guild.
pub enum Permissions {
    Admin,
    User,
    None,
}

#[async_recursion]
/// Returns the permission status of the user for the given guild.
/// Can be Admin, User or None.
pub async fn permissions_check(
    discord_token: &str,
    guild_id: &str,
    force_new: bool,
) -> Permissions {
    // No need to make a request if the token is empty.
    if discord_token.is_empty() || discord_token == "Bearer" || guild_id.is_empty() {
        return Permissions::None;
    }

    // All permissions
    let admin_permissions = 2147483647;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        match header::HeaderValue::from_str(&("Bearer ".to_owned() + discord_token)) {
            Ok(s) => s,
            Err(_) => return Permissions::None,
        },
    );

    // If it is very important to have an up to date response, we can force a new request.
    // Otherwise we can use the cache to avoid rate limits.
    let cache_mode = if force_new {
        CacheMode::NoStore
    } else {
        CacheMode::Default
    };

    let client = ClientBuilder::new(match Client::builder().default_headers(headers).build() {
        Ok(s) => s,
        Err(_) => return Permissions::None,
    })
    .with(Cache(HttpCache {
        mode: cache_mode,
        manager: CACacheManager::default(),
        options: None,
    }))
    .build();

    let res = client
        .get("https://discord.com/api/users/@me/guilds")
        .send()
        .await;
    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().await.unwrap_or("".to_string());

            let json: Value = match serde_json::from_str(&body) {
                Ok(s) => s,
                Err(_) => return Permissions::None,
            };

            // If we get rate limited, we can try again after the retry_after time.
            // This only makes sense if the retry_after time is less than ~2 seconds.
            if json["message"] == "You are being rate limited." {
                let retry_after = json["retry_after"].as_f64().unwrap_or(0.0);

                if retry_after < 2.0 && retry_after > 0.0 {
                    println!("Rate limited, retrying after {} seconds", retry_after);

                    // Wait for the retry_after time.
                    std::thread::sleep(std::time::Duration::from_secs_f64(retry_after));

                    // Try again.
                    return permissions_check(discord_token, guild_id, force_new).await;
                } else {
                    println!("Rate limited, retry_after time too long: {}", retry_after);

                    // If the retry_after time is too long, we just return None.
                    return Permissions::None;
                }
            }

            for guild in json.as_array().unwrap_or(&vec![]) {
                let current_guild_id = guild["id"].as_str().unwrap_or("0");
                let permissions = &guild["permissions"];

                // If the user is an admin of the guild, return Admin.
                // If the user is a member of the guild, return User.
                // If none of the above, we return None.
                if guild_id == current_guild_id && permissions == admin_permissions {
                    return Permissions::Admin;
                } else if guild_id == current_guild_id {
                    return Permissions::User;
                }
            }
        }
        Err(_) => {
            return Permissions::None;
        }
    }

    Permissions::None
}
