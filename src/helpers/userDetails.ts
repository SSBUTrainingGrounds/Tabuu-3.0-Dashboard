import { getRandomUserAvatar } from "./mockUsers";
import type { GuildUser } from "./types";

/**
 * Looks up a user name by ID.
 * @param users - The users to look up, as a Map of IDs and GuildUsers.
 * @param id - The ID of the user to look up.
 * @returns {string} The user name, or "Unknown User" if the user is not found.
 */
export function getUserName(users: Map<string, GuildUser>, id: string): string {
    const user = users.get(id);

    if (user) {
        return user.name;
    } else {
        return "Unknown User";
    }
}

/**
 * Looks up a user avatar by ID.
 * @param users - The users to look up, as a Map of IDs and GuildUsers.
 * @param id - The ID of the user to look up.
 * @returns - The user avatar, or a random default avatar if the user is not found.
 */
export function getUserAvatar(users: Map<string, GuildUser>, id: string): string {
    const user = users.get(id);

    if (user && user.avatar) {
        return "https://cdn.discordapp.com/avatars/" + id + "/" + user.avatar + ".png";
    } else {
        return getRandomUserAvatar();
    }
}
