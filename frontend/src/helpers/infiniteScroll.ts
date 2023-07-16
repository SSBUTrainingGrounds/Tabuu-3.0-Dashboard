/**
 * Handles infinite scrolling of items.
 * @param displayItems The items to display.
 * @param rawItems The raw item list.
 * @param page The current page number.
 * @param increaseBy How many items to increase by.
 * @param currentSearch The current search query, used to determine the displayed items.
 * @returns The new page number.
 */
export function infiniteScroll(
    displayItems: any[],
    rawItems: any[],
    page: number,
    increaseBy: number,
    currentSearch: string
): number {
    // Calculates the bottom of the page with a generous offset.
    const bottomHit =
        window.scrollY + window.innerHeight + document.body.offsetHeight + 250 >= document.body.scrollHeight;

    console.log(window.scrollY + window.innerHeight + document.body.offsetHeight + 250, document.body.scrollHeight);

    // If we have displayed all items, or we are searching something, or we are not at the bottom of the page, return.
    if (displayItems.length >= rawItems.length || currentSearch !== "" || !bottomHit) {
        return page;
    }

    const start = (page - 1) * increaseBy;
    const end = page * increaseBy > rawItems.length ? rawItems.length : page * increaseBy;

    if (rawItems.length > 0) {
        const items = rawItems.slice(start, end);
        items.forEach((item: any) => {
            displayItems.push(item);
        });
    }

    return page + 1;
}
