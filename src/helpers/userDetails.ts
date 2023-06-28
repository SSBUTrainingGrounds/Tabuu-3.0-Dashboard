import { getRandomUserAvatar } from "./mockUsers";

export function getUserName(users: Map<string, Object>, id: string): string {
    const user = users.get(id);

    if (user) {
        // @ts-ignore
        return user.name;
    } else {
        return "Unknown User";
    }
}

export function getUserAvatar(users: Map<string, Object>, id: string): string {
    const user = users.get(id);

    // @ts-ignore
    if (user && user.avatar) {
        // @ts-ignore
        return "https://cdn.discordapp.com/avatars/" + id + "/" + user.avatar + ".png";
    } else {
        return getRandomUserAvatar();
    }
}
