/**
 * This function sorts a table based on a column.
 * @param table The table to sort.
 * @param column The column to sort by.
 * @param ascending_obj The object containing the ascending values for each column.
 */
export function sortTable(table: any[], column: string, ascending_obj: any) {
    const ascending = ascending_obj[column];

    table.sort((a, b) => {
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
