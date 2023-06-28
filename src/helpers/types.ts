export interface GuildUser {
    // We currently don't need more.
    name: string;
    avatar: string;
}

export interface Macro {
    name: string;
    payload: string;
    author: string;
    uses: number;
}
