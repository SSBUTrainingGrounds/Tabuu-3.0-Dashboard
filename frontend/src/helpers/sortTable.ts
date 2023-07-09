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
