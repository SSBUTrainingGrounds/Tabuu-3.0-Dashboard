export function displayMacroPayload(payload: string): string {
    if (payload.length > 200) {
        return payload.substring(0, 200) + "...";
    } else {
        return payload;
    }
}
