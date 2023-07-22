import { getDefaultUserAvatar } from "./mockUsers";
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
        return getDefaultUserAvatar(id);
    }
}

/**
 * Fetches a user manually, if it isn't already cached.
 * @param users The cache of users.
 * @param id The ID of the user to fetch.
 */
export async function fetchUser(users: Map<string, GuildUser>, id: string) {
    if (users.has(id)) {
        return;
    }

    const url = new URL(import.meta.env.VITE_API_URL);
    url.port = import.meta.env.VITE_API_PORT;
    url.pathname = "/api/user/" + id;

    await fetch(url, {
        method: "GET",
        headers: {
            Authorization: `Bearer ${localStorage.getItem("discordToken")}`
        }
    })
        .then((response) => response.json())
        .then((data) => {
            users.set(id, {
                name: data.username,
                avatar: data.avatar
            });
        });
}
