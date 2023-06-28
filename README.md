# dashboard-frontend

Install dependencies

```bash
npm install
```

Set up `.env` file

```bash
# The ID of the guild for the leaderboard
VITE_GUILD_ID = "1234567890"
# The Token of the bot, used for requests
DISCORD_TOKEN = "Example.Token"
# The location of the database
DATABASE_LOCATION = "./path/to/database.db"
```

Start express server

```
npm start
```

Start frontend

```
npm run dev -- --open
```
