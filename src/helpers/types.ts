/**
 * Used internally to represent a user.
 * Stored in a map with the key being the user ID.
 */
export interface GuildUser {
    // We currently don't need more.
    name: string;
    avatar: string | null;
}

/**
 * A macro command that can be executed by a user.
 */
export interface Macro {
    name: string;
    payload: string;
    author: string;
    uses: number;
}

/**
 * The data returned by the Discord API when a user logs in.
 */
export interface LoggedInUser {
    id: string;
    username: string;
    discriminator: string;
    avatar: string | null;
    bot: boolean;
    system: boolean;
    mfa_enabled: boolean;
    locale: string;
    verified: boolean;
    email: string | null;
    flags: number;
    premium_type: number;
    public_flags: number;
}
