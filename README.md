# dashboard-frontend

Install dependencies

```bash
npm install
```

Set up `.env` file in the server folder

```bash
# The ID of the guild for the leaderboard
GUILD_ID = "1234567890"
# The Token of the bot, used for requests
DISCORD_TOKEN = "Example.Token"
# The location of the database
DATABASE_LOCATION = "./path/to/database.db"
```

Set up `.env` file in the client folder

```bash
VITE_GUILD_ID = "775528948683374632"
```

Start rocket.rs server, in the server folder

```
cargo run --release
```

Start frontend

```
npm run dev -- --open
```
