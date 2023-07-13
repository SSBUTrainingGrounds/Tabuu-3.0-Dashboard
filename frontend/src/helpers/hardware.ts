/**
 * Gets the average of the given array of numbers.
 * @param arr The array of numbers.
 * @returns The average.
 */
export function getAverage(arr: number[]): number {
    if (!arr || arr.length === 0) return 0;

    return arr.reduce((a, b) => a + b) / arr.length;
}

/**
 * Returns a readable string of the given bytes, in GB.
 * @param bytes The bytes to convert.
 * @returns The readable string.
 */
export function getGBString(bytes: number): string {
    return (
        (bytes / 1024 / 1024 / 1024).toLocaleString("en", {
            maximumFractionDigits: 2,
            minimumFractionDigits: 2
        }) + " GB"
    );
}
