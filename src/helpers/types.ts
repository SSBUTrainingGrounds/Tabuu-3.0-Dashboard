export interface GuildUser {
    // We currently don't need more.
    name: string;
    avatar: string | null;
}

export interface Macro {
    name: string;
    payload: string;
    author: string;
    uses: number;
}

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
