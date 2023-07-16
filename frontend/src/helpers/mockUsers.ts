/**
 * This function returns a mock user name.
 * @returns {string} A random user name.
 */
export function getRandomUserName(): string {
    // Mainly used for testing to avoid spamming the discord API.
    const usernameList = [
        "Aardvark",
        "Albatross",
        "Alligator",
        "Alpaca",
        "Ant",
        "Anteater",
        "Antelope",
        "Ape",
        "Armadillo",
        "Donkey",
        "Baboon",
        "Badger",
        "Barracuda",
        "Bat",
        "Bear",
        "Beaver",
        "Bee",
        "Bison",
        "Boar",
        "Buffalo",
        "Butterfly",
        "Camel",
        "Capybara",
        "Caribou",
        "Cassowary",
        "Cat",
        "Caterpillar",
        "Cattle"
    ];

    const username = usernameList[Math.floor(Math.random() * usernameList.length)];

    return username;
}

/**
 * This function returns the default user avatar. If no userID is provided, a random avatar is returned.
 * @param {string} userID - The ID of the user to get the avatar for.
 * @returns {string} The default user avatar.
 */
export function getDefaultUserAvatar(userID: string | undefined = undefined): string {
    const avatarList = [
        "https://cdn.discordapp.com/embed/avatars/0.png",
        "https://cdn.discordapp.com/embed/avatars/1.png",
        "https://cdn.discordapp.com/embed/avatars/2.png",
        "https://cdn.discordapp.com/embed/avatars/3.png",
        "https://cdn.discordapp.com/embed/avatars/4.png",
        "https://cdn.discordapp.com/embed/avatars/5.png"
    ];

    if (userID) {
        return avatarList[Number((BigInt(userID) >> BigInt(22)) % BigInt(avatarList.length))];
    } else {
        return avatarList[Math.floor(Math.random() * avatarList.length)];
    }
}
