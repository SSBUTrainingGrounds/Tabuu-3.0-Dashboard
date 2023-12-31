extern crate reqwest;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{header, Client};
use reqwest_middleware::ClientBuilder;

use serde::Serialize;
use serde_json::Value;

use crate::types::{FetchedUser, RawGuildUser, RawUser};

/// Gets all the users in a guild.
/// Returns an empty vector if the request fails.
/// If a user cannot be parsed, it will insert an "empty" user.
/// Otherwise returns a vector of users.
pub async fn get_users(discord_token: &str, guild_id: &str) -> Vec<RawGuildUser> {
    let mut users: Vec<RawGuildUser> = vec![];

    let mut after: String = "0".to_string();
    let mut keep_going = true;

    let mut headers = header::HeaderMap::new();
    // We need to make this request as a bot, so we need to add the "Bot" prefix.
    // The bot needs to be in the guild to make this request, and have the "Server Member Intent" turned on.
    headers.insert(
        header::AUTHORIZATION,
        match header::HeaderValue::from_str(&("Bot ".to_owned() + discord_token)) {
            Ok(s) => s,
            Err(_) => return users,
        },
    );

    let client = ClientBuilder::new(match Client::builder().default_headers(headers).build() {
        Ok(s) => s,
        Err(_) => return users,
    })
    .with(Cache(HttpCache {
        mode: CacheMode::Default,
        manager: CACacheManager::default(),
        options: HttpCacheOptions::default(),
    }))
    .build();

    // We need to make multiple requests to get all the users.
    // Each request returns 1000 users, so we need to keep making requests until we get less than 1000 users.
    // This could be an issue for the rate limit, so we can use the cache to avoid this.
    while keep_going {
        let res = client
            .get(
                &("https://discord.com/api/guilds/".to_owned()
                    + guild_id
                    + "/members?limit=1000&after="
                    + after.as_ref()),
            )
            .send()
            .await;
        #[allow(unused_assignments)]
        let mut body = String::new();

        match res {
            Ok(res) => {
                body = res.text().await.unwrap_or("".to_string());

                let json: Value = match serde_json::from_str(&body) {
                    Ok(s) => s,
                    Err(_) => return users,
                };

                // We need to keep track of the last user in the array, so we can use it to get the next 1000 users in the request.
                let last = json.as_array().unwrap_or(&vec![Value::Null]).len() - 1;

                after = json.as_array().unwrap_or(&vec![])[last]["user"]["id"]
                    .as_str()
                    .unwrap_or("0")
                    .to_string();

                for user in json.as_array().unwrap_or(&vec![]) {
                    let user: RawGuildUser = match serde_json::from_value(user.clone()) {
                        Ok(s) => s,
                        Err(_) => RawGuildUser {
                            avatar: Some("".to_string()),
                            communication_disabled_until: Some("".to_string()),
                            deaf: false,
                            flags: 0,
                            joined_at: "".to_string(),
                            mute: false,
                            nick: Some("".to_string()),
                            pending: false,
                            premium_since: Some("".to_string()),
                            roles: vec![],
                            user: RawUser {
                                accent_color: Some(0),
                                avatar: Some("".to_string()),
                                avatar_decoration: Some("".to_string()),
                                banner: Some("".to_string()),
                                banner_color: Some("".to_string()),
                                bot: Some(false),
                                discriminator: "".to_string(),
                                display_name: Some("".to_string()),
                                flags: 0,
                                global_name: Some("".to_string()),
                                id: "".to_string(),
                                public_flags: 0,
                                username: "".to_string(),
                            },
                        },
                    };
                    users.push(user);
                }

                if json.as_array().unwrap_or(&vec![]).len() < 1000 {
                    keep_going = false;
                }
            }
            Err(_) => {
                keep_going = false;
            }
        }
    }

    users
}

/// Fetches a single user from the Discord API.
/// Returns None if the request fails, or the user cannot be parsed.
/// Otherwise returns the user.
pub async fn fetch_single_user(discord_token: &str, user_id: &str) -> Option<FetchedUser> {
    let mut headers = header::HeaderMap::new();
    // Same as the get_users function, we need to make this request as a bot.
    headers.insert(
        header::AUTHORIZATION,
        match header::HeaderValue::from_str(&("Bot ".to_owned() + discord_token)) {
            Ok(s) => s,
            Err(_) => return None,
        },
    );

    let client = ClientBuilder::new(match Client::builder().default_headers(headers).build() {
        Ok(s) => s,
        Err(_) => return None,
    })
    .with(Cache(HttpCache {
        mode: CacheMode::Default,
        manager: CACacheManager::default(),
        options: HttpCacheOptions::default(),
    }))
    .build();

    let res = client
        .get(&("https://discord.com/api/users/".to_owned() + user_id))
        .send()
        .await;

    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().await.unwrap_or("".to_string());

            let json: Value = match serde_json::from_str(&body) {
                Ok(s) => s,
                Err(_) => return None,
            };

            let user: FetchedUser = match serde_json::from_value(json) {
                Ok(s) => s,
                Err(_) => return None,
            };

            Some(user)
        }

        Err(_) => None,
    }
}

pub async fn fetch_me(discord_token: &str) -> Option<RawUser> {
    // This request is made as a user, "Bearer" prefix.
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        match header::HeaderValue::from_str(&("Bearer ".to_owned() + discord_token)) {
            Ok(s) => s,
            Err(_) => return None,
        },
    );

    let client = ClientBuilder::new(match Client::builder().default_headers(headers).build() {
        Ok(s) => s,
        Err(_) => return None,
    })
    .with(Cache(HttpCache {
        mode: CacheMode::NoStore,
        manager: CACacheManager::default(),
        options: HttpCacheOptions::default(),
    }))
    .build();

    let res = client.get("https://discord.com/api/users/@me").send().await;

    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().await.unwrap_or("".to_string());

            let json: Value = match serde_json::from_str(&body) {
                Ok(s) => s,
                Err(_) => return None,
            };

            let user: RawUser = match serde_json::from_value(json) {
                Ok(s) => s,
                Err(_) => return None,
            };

            Some(user)
        }

        Err(_) => None,
    }
}

/// Gets the JSON string of a serializable object.
/// Returns an empty array string if the object cannot be serialized.
pub fn get_json_string(return_type: impl Sized + Serialize) -> String {
    match serde_json::to_string(&return_type) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            String::from("[]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_string() {
        let test_struct = RawGuildUser {
            avatar: Some("".to_string()),
            communication_disabled_until: Some("".to_string()),
            deaf: false,
            flags: 0,
            joined_at: "".to_string(),
            mute: false,
            nick: Some("".to_string()),
            pending: false,
            premium_since: Some("".to_string()),
            roles: vec![],
            user: RawUser {
                accent_color: Some(0),
                avatar: Some("".to_string()),
                avatar_decoration: Some("".to_string()),
                banner: Some("".to_string()),
                banner_color: Some("0".to_string()),
                bot: Some(false),
                discriminator: "".to_string(),
                display_name: Some("".to_string()),
                flags: 0,
                global_name: Some("".to_string()),
                id: "".to_string(),
                public_flags: 0,
                username: "".to_string(),
            },
        };

        let json_string = get_json_string(test_struct);

        assert_eq!(
            json_string,
            "{\"avatar\":\"\",\"communication_disabled_until\":\"\",\"deaf\":false,\"flags\":0,\"joined_at\":\"\",\"mute\":false,\"nick\":\"\",\"pending\":false,\"premium_since\":\"\",\"roles\":[],\"user\":{\"accent_color\":0,\"avatar\":\"\",\"avatar_decoration\":\"\",\"banner\":\"\",\"banner_color\":\"0\",\"bot\":false,\"discriminator\":\"\",\"display_name\":\"\",\"flags\":0,\"global_name\":\"\",\"id\":\"\",\"public_flags\":0,\"username\":\"\"}}"
        );

        let test_struct = RawGuildUser {
            avatar: Some("Some Avatar".to_string()),
            communication_disabled_until: Some("1234".to_string()),
            deaf: true,
            flags: 538976288,
            joined_at: "242823".to_string(),
            mute: true,
            nick: Some("Nickname!".to_string()),
            pending: true,
            premium_since: Some("342342".to_string()),
            roles: vec![
                "Role 1".to_string(),
                "Role 2".to_string(),
                "Role 3".to_string(),
            ],
            user: RawUser {
                accent_color: Some(423231),
                avatar: Some("Another Avatar".to_string()),
                avatar_decoration: Some("Whatever".to_string()),
                banner: Some("Some banner".to_string()),
                banner_color: Some("43234234".to_string()),
                bot: Some(true),
                discriminator: "0".to_string(),
                display_name: Some("A display name".to_string()),
                flags: 423423243,
                global_name: Some("Global Name".to_string()),
                id: "42332323333".to_string(),
                public_flags: 8,
                username: "USERNAME".to_string(),
            },
        };

        let json_string = get_json_string(test_struct);

        assert_eq!(
            json_string,
            "{\"avatar\":\"Some Avatar\",\"communication_disabled_until\":\"1234\",\"deaf\":true,\"flags\":538976288,\"joined_at\":\"242823\",\"mute\":true,\"nick\":\"Nickname!\",\"pending\":true,\"premium_since\":\"342342\",\"roles\":[\"Role 1\",\"Role 2\",\"Role 3\"],\"user\":{\"accent_color\":423231,\"avatar\":\"Another Avatar\",\"avatar_decoration\":\"Whatever\",\"banner\":\"Some banner\",\"banner_color\":\"43234234\",\"bot\":true,\"discriminator\":\"0\",\"display_name\":\"A display name\",\"flags\":423423243,\"global_name\":\"Global Name\",\"id\":\"42332323333\",\"public_flags\":8,\"username\":\"USERNAME\"}}"
        );
    }
}
