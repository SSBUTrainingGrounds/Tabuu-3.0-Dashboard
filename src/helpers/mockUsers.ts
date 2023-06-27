/**
 * This function returns a mock user name.
 * @returns {string} A random user name.
 */
export function getRandomUserName(): string {
    // It would be better to use a real user name and avatar, but this is a quick and easy way to get a random user.
    // So we don't have to call the discord API over and over again.

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
 * This function returns a mock user avatar.
 * @returns {string} A random user avatar.
 */
export function getRandomUserAvatar(): string {
    const avatarList = [
        "https://cdn.discordapp.com/embed/avatars/0.png",
        "https://cdn.discordapp.com/embed/avatars/1.png",
        "https://cdn.discordapp.com/embed/avatars/2.png",
        "https://cdn.discordapp.com/embed/avatars/3.png",
        "https://cdn.discordapp.com/embed/avatars/4.png",
        "https://cdn.discordapp.com/embed/avatars/5.png"
    ];

    return avatarList[Math.floor(Math.random() * avatarList.length)];
}
