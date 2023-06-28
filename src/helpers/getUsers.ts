import "dotenv/config";
import type { RawGuildUser } from "./types";

/**
 * Fetches all users from a guild.
 * @param guildID - The ID of the guild to fetch users from.
 * @param botToken - The bot token to use for authentication.
 * @returns {Promise<RawGuildUser[]>} An array of users.
 */
export async function getUsers(guildID: string, botToken: string): Promise<RawGuildUser[]> {
    // TODO: Improve performance.

    const users: RawGuildUser[] = [];

    let after = "0";
    let keepGoing = true;

    while (keepGoing) {
        await fetch(`https://discord.com/api/guilds/${guildID}/members?limit=1000&after=${after}`, {
            headers: {
                Authorization: `Bot ${botToken}`
            }
        })
            .then((res) => res.json())
            .then((data) => {
                if (data.message) {
                    // These are most likely either rate limits, or wrong token/guild ID.
                    console.log(data.message);
                    keepGoing = false;
                } else {
                    users.push(...data);
                    if (data.length < 1000) {
                        keepGoing = false;
                    }
                    after = data[data.length - 1].user.id;
                }
            });
    }

    return users;
}
