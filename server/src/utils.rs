extern crate reqwest;
use reqwest::header;
use serde_json::Value;

use crate::types::{FetchedUser, RawGuildUser};

pub async fn admin_check(discord_token: &str, guild_id: &str) -> bool {
    // TODO: Remove all of those unwraps.

    let admin_permissions = 2147483647;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&("Bearer ".to_owned() + discord_token))
            .unwrap_or(header::HeaderValue::from_str("").unwrap()),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .get("https://discord.com/api/users/@me/guilds")
        .send()
        .await;
    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().await.unwrap_or("".to_string());

            let json: Value = serde_json::from_str(&body).unwrap();

            for guild in json.as_array().unwrap_or(&vec![]) {
                let current_guild_id = guild["id"].as_str().unwrap_or("0");
                let permissions = &guild["permissions"];

                if guild_id == current_guild_id && permissions == admin_permissions {
                    return true;
                }
            }
        }
        Err(_) => {
            return false;
        }
    }

    false
}

pub async fn get_users(discord_token: &str, guild_id: &str) -> Vec<RawGuildUser> {
    let mut users: Vec<RawGuildUser> = vec![];

    let mut after: String = "0".to_string();
    let mut keep_going = true;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&("Bot ".to_owned() + discord_token))
            .unwrap_or(header::HeaderValue::from_str("").unwrap()),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

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

                let json: Value = serde_json::from_str(&body).unwrap();

                let last = json.as_array().unwrap_or(&vec![]).len() - 1;

                after = json.as_array().unwrap()[last]["user"]["id"]
                    .as_str()
                    .unwrap_or("0")
                    .to_string();

                for user in json.as_array().unwrap_or(&vec![]) {
                    let user: RawGuildUser = serde_json::from_value(user.clone()).unwrap();
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

pub async fn fetch_single_user(discord_token: &str, user_id: &str) -> Option<FetchedUser> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&("Bot ".to_owned() + discord_token))
            .unwrap_or(header::HeaderValue::from_str("").unwrap()),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .get(&("https://discord.com/api/users/".to_owned() + user_id))
        .send()
        .await;

    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().await.unwrap_or("".to_string());

            let json: Value = serde_json::from_str(&body).unwrap();

            let user: FetchedUser = serde_json::from_value(json).unwrap();

            Some(user)
        }

        Err(_) => None,
    }
}
