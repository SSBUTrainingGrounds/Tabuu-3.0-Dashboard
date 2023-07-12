/**
 * This function returns a list of emoji urls from a string of characters.
 * @param characters The string of characters to get the emoji urls from.
 * @returns A list of emoji urls.
 */
export function getCharacters(characters: string): string[] {
    if (!characters) return [];

    return characters.split(" ").map((c) => {
        c = c.split(":")[2];
        c = `https://cdn.discordapp.com/emojis/${c.slice(0, c.length - 1)}.png`;
        return c;
    });
}
