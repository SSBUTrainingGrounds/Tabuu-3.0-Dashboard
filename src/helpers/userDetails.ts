import { getRandomUserAvatar } from "./mockUsers";
import type { GuildUser } from "./types";

export function getUserName(users: Map<string, GuildUser>, id: string): string {
    const user = users.get(id);

    if (user) {
        return user.name;
    } else {
        return "Unknown User";
    }
}

export function getUserAvatar(users: Map<string, GuildUser>, id: string): string {
    const user = users.get(id);

    if (user && user.avatar) {
        return "https://cdn.discordapp.com/avatars/" + id + "/" + user.avatar + ".png";
    } else {
        return getRandomUserAvatar();
    }
}
