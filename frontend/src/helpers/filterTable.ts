import type { GuildUser } from "./types";

/**
 * This function filters a user table based on a query string.
 * @param data The user table data.
 * @param users_dict The user dictionary, fetched at the start.
 * @param query The query string.
 * @returns The filtered user table data.
 */
export function filterTable(data: any[], users_dict: Map<string, GuildUser>, query: string): any[] {
    if (!query) return data;

    const matching_ids: string[] = [];

    for (const [id, user] of users_dict) {
        if (user.name.toLowerCase().includes(query.toLowerCase())) {
            matching_ids.push(id);
        }
    }

    return data.filter((row) => {
        // If we have a winner and loser, we're dealing with a match, and we need to check both winner and loser ids.
        if (row.winner_id && row.loser_id) {
            return matching_ids.includes(row.winner_id) || matching_ids.includes(row.loser_id);
        } else if (row.user_id) {
            // Otherwise it's a user, and we need to check either user_id, or id.
            return matching_ids.includes(row.user_id);
        }
        return matching_ids.includes(row.id);
    });
}
