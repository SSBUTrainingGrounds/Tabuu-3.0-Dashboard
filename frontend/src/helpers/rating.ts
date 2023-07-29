export function getRatingChangeText(ratingChange: number): string {
    if (ratingChange > 0) {
        return `↑${ratingChange.toLocaleString("en", {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2
        })}`;
    } else {
        return `↓${Math.abs(ratingChange).toLocaleString("en", {
            minimumFractionDigits: 2,
            maximumFractionDigits: 2
        })}`;
    }
}
