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

/**
 * Extra global user data returned by the Discord API, when a Guild User is fetched.
 */
export interface RawUser {
    accent_color: number | null;
    avatar: string | null;
    avatar_decoration: string | null;
    banner: string | null;
    banner_color: string | null;
    bot: boolean;
    discriminator: string;
    display_name: string | null;
    flags: number;
    global_name: string | null;
    id: string;
    public_flags: number;
    username: string;
}

/**
 * The data returned by the Discord API when a user is fetched by the bot.
 */
export interface RawGuildUser {
    avatar: string | null;
    communication_disabled_until: number | null;
    deaf: boolean;
    flags: number;
    joined_at: string;
    mute: boolean;
    nick: string | null;
    pending: boolean;
    premium_since: string | null;
    roles: string[];
    user: RawUser;
}
