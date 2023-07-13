export function getAverage(arr: number[]): number {
    if (!arr || arr.length === 0) return 0;

    return arr.reduce((a, b) => a + b) / arr.length;
}

export function getGBString(bytes: number): string {
    return (
        (bytes / 1024 / 1024 / 1024).toLocaleString("en", {
            maximumFractionDigits: 2,
            minimumFractionDigits: 2
        }) + " GB"
    );
}
