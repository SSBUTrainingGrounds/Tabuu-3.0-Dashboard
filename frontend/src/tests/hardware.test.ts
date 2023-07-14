import { describe, it, expect } from "vitest";
import { getAverage, getGBString, celciusToFahrenheit } from "@/helpers/hardware";

describe("getAverage", () => {
    it("should return 0 if the array is empty", () => {
        expect(getAverage([])).toBe(0);
    });

    it("should return the average of the array", () => {
        expect(getAverage([1, 2, 3, 4, 5])).toBe(3);
    });
});

describe("getGBString", () => {
    it("should return the correct string", () => {
        expect(getGBString(1024 * 1024 * 1024)).toBe("1.00 GB");
    });

    it("should return the correct string", () => {
        expect(getGBString(1024 * 1024 * 1024 * 2)).toBe("2.00 GB");
    });

    it("should return the correct string", () => {
        expect(getGBString(1024 * 1024 * 1024 * 2.5)).toBe("2.50 GB");
    });
});

describe("celciusToFahrenheit", () => {
    it("should return the correct value", () => {
        expect(celciusToFahrenheit(0)).toBe(32);
    });

    it("should return the correct value", () => {
        expect(celciusToFahrenheit(100)).toBe(212);
    });

    it("should return the correct value", () => {
        expect(celciusToFahrenheit(50)).toBe(122);
    });
});
