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
    display_payload: string;
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
 * The data returned by our API for the hardware stats.
 */
export interface HwStats {
    uptime: number;
    os_name: string;
    cpu_name: string;
    cpu_freq: number[];
    avg_cpu_freq: number;
    cpu_usage: number[];
    avg_cpu_usage: number;
    cpu_temp_c: number[];
    avg_cpu_temp_c: number;
    cpu_temp_f: number[];
    avg_cpu_temp_f: number;
    cpu_cores: number[];
    ram_total: number;
    ram_used: number;
    ram_free: number;
    ram_percentage: number;
    ram_readable_str: string;
    swap_total: number;
    swap_used: number;
    swap_free: number;
    swap_percentage: number;
    swap_readable_str: string;
    disks: number[][];
}
