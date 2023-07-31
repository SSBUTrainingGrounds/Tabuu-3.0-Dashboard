/// This function converts a string of raw emojis to a vector of discord emoji image links.
pub fn get_emojis_from_str(input_str: String) -> Vec<String> {
    let mut emoji_vec: Vec<String> = Vec::new();

    input_str.split(' ').for_each(|x| {
        if x.starts_with("<:") || x.starts_with("<a:") {
            let id = x.split(':').collect::<Vec<&str>>()[2].replace(['>', '<'], "");

            emoji_vec.push("https://cdn.discordapp.com/emojis/".to_string() + &id + ".png");
        }
    });

    emoji_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_emoji_conversion() {
        let test_input = "<:test:123456789>".to_string();

        let expected_output = vec!["https://cdn.discordapp.com/emojis/123456789.png".to_string()];

        assert_eq!(get_emojis_from_str(test_input), expected_output);
    }

    #[test]
    fn test_multiple_emoji_conversion() {
        let test_input =
            "<:test:123456789> <:test:123456789> <:hello:432783478> <:hi!:243787324842372834>"
                .to_string();

        let expected_output = vec![
            "https://cdn.discordapp.com/emojis/123456789.png".to_string(),
            "https://cdn.discordapp.com/emojis/123456789.png".to_string(),
            "https://cdn.discordapp.com/emojis/432783478.png".to_string(),
            "https://cdn.discordapp.com/emojis/243787324842372834.png".to_string(),
        ];

        assert_eq!(get_emojis_from_str(test_input), expected_output);
    }
}
