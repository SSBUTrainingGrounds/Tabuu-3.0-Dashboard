import { describe, it, expect } from "vitest";
import { getCharacters } from "../helpers/characterEmojis";

describe("Character Emojis", () => {
    describe("getCharacters", () => {
        it("should return a list of characters", () => {
            const characters =
                "<:Mario:929067419861913680> <:DrMario:929131885525868595> <:KingDedede:929070473311027300> <:Shulk:929086965897363538>";

            const result = getCharacters(characters);

            expect(result).toEqual([
                "https://cdn.discordapp.com/emojis/929067419861913680.png",
                "https://cdn.discordapp.com/emojis/929131885525868595.png",
                "https://cdn.discordapp.com/emojis/929070473311027300.png",
                "https://cdn.discordapp.com/emojis/929086965897363538.png"
            ]);
        });

        it("should return an empty list if no characters are found", () => {
            const characters = "";

            const result = getCharacters(characters);

            expect(result).toEqual([]);
        });
    });
});
