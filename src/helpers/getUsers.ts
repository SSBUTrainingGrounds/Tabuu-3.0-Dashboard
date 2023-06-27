import "dotenv/config";

export async function getUsers(guildID: string, botToken: string): Promise<Object[]> {
    // TODO: Improve performance.

    const users: Object[] = [];

    let after = "0";
    let keepGoing = true;

    while (keepGoing) {
        await fetch(`https://discord.com/api/guilds/${guildID}/members?limit=1000&after=${after}`, {
            headers: {
                Authorization: `Bot ${botToken}`
            }
        })
            .then((res) => res.json())
            .then((data) => {
                if (data.message) {
                    console.log(data.message);
                    keepGoing = false;
                } else {
                    users.push(...data);
                    if (data.length < 1000) {
                        keepGoing = false;
                    }
                    after = data[data.length - 1].user.id;
                }
            });
    }

    return users;
}
