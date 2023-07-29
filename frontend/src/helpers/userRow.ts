/**
 * Finds a user in a table and returns the row number.
 * @param user_id The user id to search for.
 * @param table The table to search in.
 * @returns The row number of the user, or -1 if the user is not found.
 */
export function getUserRow(user_id: string, table: any[]): number {
    for (let i = 0; i < table.length; i++) {
        if (table[i].user_id === user_id || table[i].id === user_id) {
            return i;
        }
    }
    return -1;
}

/**
 * Scrolls the main user-table to the given row.
 * @param row The row to scroll to.
 */
export function scrollToRow(row: number) {
    if (row < 0) {
        return;
    }

    const rows = document.querySelectorAll("#user-table .content");

    if (rows.length > 0) {
        const rowElement = rows[row] as HTMLElement;
        if (rowElement) {
            rowElement.scrollIntoView({
                behavior: "smooth",
                block: "center",
                inline: "center"
            });
        } else {
            // The row is not in the current page, so we scroll to the bottom and try again
            const table = document.querySelector("#user-table") as HTMLElement;
            table.scrollIntoView({
                behavior: "smooth",
                block: "end",
                inline: "end"
            });
            setTimeout(() => {
                scrollToRow(row);
            }, 100);
        }
    }
}
