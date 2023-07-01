import { describe, it, expect } from "vitest";
import { getUserName, getUserAvatar } from "../helpers/userDetails";
import type { GuildUser } from "@/helpers/types";

describe("User Details", () => {
    describe("getUserName", () => {
        it("should return the user name if the user exists", () => {
            const users = new Map<string, GuildUser>();
            users.set("123", {
                name: "Test User",
                avatar: "abc123"
            });

            const result = getUserName(users, "123");

            expect(result).toBe("Test User");
        });

        it("should return 'Unknown User' if the user does not exist", () => {
            const users = new Map<string, GuildUser>();

            const result = getUserName(users, "123");

            expect(result).toBe("Unknown User");
        });
    });

    describe("getUserAvatar", () => {
        it("should return the user avatar if the user exists", () => {
            const users = new Map<string, GuildUser>();
            users.set("123", {
                name: "Test User",
                avatar: "abc123"
            });

            const result = getUserAvatar(users, "123");

            expect(result).toBe("https://cdn.discordapp.com/avatars/123/abc123.png");
        });

        it("should return a random avatar if the user does not exist", () => {
            const users = new Map<string, GuildUser>();

            const result = getUserAvatar(users, "123");

            expect(result).toMatch(/https:\/\/cdn.discordapp.com\/embed\/avatars\/\d+.png/);
        });
    });
});
