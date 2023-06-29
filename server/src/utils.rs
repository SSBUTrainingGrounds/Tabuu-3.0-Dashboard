extern crate reqwest;
use reqwest::header;
use serde_json::Value;

pub fn admin_check(discord_token: &str, guild_id: &str) -> bool {
    // TODO: Remove all of those unwraps.

    let admin_permissions = 2147483647;

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&("Bearer ".to_owned() + discord_token))
            .unwrap_or(header::HeaderValue::from_str("").unwrap()),
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let res = client
        .get("https://discord.com/api/users/@me/guilds")
        .send();
    #[allow(unused_assignments)]
    let mut body = String::new();

    match res {
        Ok(res) => {
            body = res.text().unwrap_or("".to_string());

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
