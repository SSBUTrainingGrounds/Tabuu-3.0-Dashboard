/**
 * This function sorts a table based on a column.
 * @param displayTable The table to sort.
 * @param column The column to sort by.
 * @param ascending_obj The object containing the ascending values for each column.
 */
export function sortTable(displayTable: any[], column: string, ascending_obj: any) {
    const ascending = ascending_obj[column];

    displayTable.sort((a, b) => {
        if (a[column] < b[column]) {
            return ascending ? -1 : 1;
        }
        if (a[column] > b[column]) {
            return ascending ? 1 : -1;
        }

        return 0;
    });

    ascending_obj[column] = !ascending;
}

/**
 * This function sorts a table based on a column. This function is used for tables with infinite scroll.
 * @param displayTable The table to sort.
 * @param rawTable The raw table to sort.
 * @param column The column to sort by.
 * @param ascending_obj The object containing the ascending values for each column.
 */
export function sortDisplayTable(displayTable: any[], rawTable: any[], column: string, ascending_obj: any): any[] {
    const ascending = ascending_obj[column];
    const tableSize = displayTable.length;

    rawTable.sort((a, b) => {
        if (a[column] < b[column]) {
            return ascending ? -1 : 1;
        }
        if (a[column] > b[column]) {
            return ascending ? 1 : -1;
        }
        return 0;
    });

    displayTable = rawTable.slice(0, tableSize);

    ascending_obj[column] = !ascending;

    return displayTable;
}
