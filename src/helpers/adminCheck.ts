export async function adminCheck(discordToken: string, guildID: string): Promise<boolean> {
    // TODO: Cache this result.

    const adminPermissions = 2147483647;

    let guilds: any[] = [];

    await fetch("https://discord.com/api/users/@me/guilds", {
        headers: {
            Authorization: `Bearer ${discordToken}`
        }
    })
        .then((res) => res.json())
        .then((data) => {
            if (data.message) {
                console.log(data.message);
            } else {
                guilds = data;
            }
        });

    for (let i = 0; i < guilds.length; i++) {
        const currentGuild = guilds[i];

        if (currentGuild.id === guildID && currentGuild.permissions === adminPermissions) {
            return true;
        }
    }

    return false;
}
