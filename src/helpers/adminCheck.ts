export async function adminCheck(discordToken: string): Promise<boolean> {
    // TODO: Cache this result.

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

        if (currentGuild.id === "739299507795132486" && currentGuild.permissions === 2147483647) {
            return true;
        }
    }

    return false;
}
