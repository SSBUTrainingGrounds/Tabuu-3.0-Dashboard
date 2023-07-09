import { describe, it, expect } from "vitest";
import { filterTable } from "@/helpers/filterTable";
import { sortTable } from "@/helpers/sortTable";
import type { GuildUser } from "@/helpers/types";

describe("Table", () => {
    describe("filterTable", () => {
        it("should return the data if the query is empty", () => {
            const data = [
                {
                    id: "123",
                    winner_id: "123",
                    loser_id: "456"
                },
                {
                    id: "456",
                    winner_id: "456",
                    loser_id: "123"
                }
            ];

            const users_dict = new Map<string, GuildUser>();
            users_dict.set("123", {
                name: "Test User",
                avatar: "abc123"
            });
            users_dict.set("456", {
                name: "Test User 2",
                avatar: "abc456"
            });

            const result = filterTable(data, users_dict, "");

            expect(result).toEqual(data);
        });

        it("should return the data if the query is undefined", () => {
            const data = [
                {
                    id: "123",
                    winner_id: "123",
                    loser_id: "456"
                },
                {
                    id: "456",
                    winner_id: "456",
                    loser_id: "123"
                }
            ];

            const users_dict = new Map<string, GuildUser>();
            users_dict.set("123", {
                name: "Test User",
                avatar: "abc123"
            });
            users_dict.set("456", {
                name: "Test User 2",
                avatar: "abc456"
            });

            // @ts-expect-error
            const result = filterTable(data, users_dict, undefined);

            expect(result).toEqual(data);
        });

        it("should return the data if the query is null", () => {
            const data = [
                {
                    id: "123",
                    winner_id: "123",
                    loser_id: "456"
                },
                {
                    id: "456",
                    winner_id: "456",
                    loser_id: "123"
                }
            ];

            const users_dict = new Map<string, GuildUser>();
            users_dict.set("123", {
                name: "Test User",
                avatar: "abc123"
            });
            users_dict.set("456", {
                name: "Test User 2",
                avatar: "abc456"
            });

            // @ts-expect-error
            const result = filterTable(data, users_dict, null);

            expect(result).toEqual(data);
        });

        it("should return search results if the query is not empty", () => {
            const data = [
                {
                    id: "123",
                    winner_id: "123",
                    loser_id: "456"
                },
                {
                    id: "456",
                    winner_id: "456",
                    loser_id: "789"
                },
                {
                    id: "789",
                    winner_id: "789",
                    loser_id: "123"
                }
            ];

            const users_dict = new Map<string, GuildUser>();
            users_dict.set("123", {
                name: "Test User",
                avatar: "abc123"
            });
            users_dict.set("456", {
                name: "Whatever else 2",
                avatar: "abc456"
            });

            const result = filterTable(data, users_dict, "Test");

            expect(result).toEqual([
                {
                    id: "123",
                    winner_id: "123",
                    loser_id: "456"
                },
                {
                    id: "789",
                    winner_id: "789",
                    loser_id: "123"
                }
            ]);
        });
    });

    describe("sortTable", () => {
        it("should sort the data by the user_id, ascending", () => {
            const table = [
                {
                    id: "123",
                    level: 1,
                    xp: 0,
                    messages: 0
                },
                {
                    id: "456",
                    level: 1,
                    xp: 0,
                    messages: 0
                }
            ];

            const ascending_obj = {
                id: true,
                level: true,
                xp: true,
                messages: true
            };

            sortTable(table, "id", ascending_obj);

            expect(table).toEqual([
                {
                    id: "123",
                    level: 1,
                    xp: 0,
                    messages: 0
                },
                {
                    id: "456",
                    level: 1,
                    xp: 0,
                    messages: 0
                }
            ]);
        });

        it("should sort the data by the user_id, descending", () => {
            const table = [
                {
                    id: "123",
                    level: 1,
                    xp: 0,
                    messages: 0
                },
                {
                    id: "456",
                    level: 1,
                    xp: 0,
                    messages: 0
                }
            ];

            const ascending_obj = {
                id: false,
                level: true,
                xp: true,
                messages: true
            };

            sortTable(table, "id", ascending_obj);

            expect(table).toEqual([
                {
                    id: "456",
                    level: 1,
                    xp: 0,
                    messages: 0
                },
                {
                    id: "123",
                    level: 1,
                    xp: 0,
                    messages: 0
                }
            ]);
        });
    });
});
