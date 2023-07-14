# Tabuu 3.0 Dashboard

Simple dashboard for the [Tabuu 3.0 Discord Bot](https://github.com/SSBUTrainingGrounds/Tabuu-3.0).  
Written in [Vue](https://vuejs.org/) and the server is powered by [Rocket](https://rocket.rs/).

## Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/)

## Installation & Usage

### Frontend

_These steps take place in the `./frontend/` directory_

1. Set up the `./frontend/.env` file

```bash
# The address to send requests to
VITE_API_URL = "http://127.0.0.1"
# The port to send requests to
VITE_API_PORT = "8080"
# The Discord OAuth2 URL.
# Visit your Application page and use the OAuth2 URL Generator.
# Check the "identify", "guilds" and "guilds.members.read" scopes.
# Make sure the response type is "token".
VITE_DISCORD_LOGIN_URL = "https://discord.com/api/oauth2/authorize?client_id=123456789&redirect_uri=http%3A%2F%2F127.0.0.1%3A80&response_type=token&scope=identify%20guilds.members.read%20guilds"
```

2. Install dependencies

```bash
npm install
```

3. **Start the frontend**

```
npm run dev -- --open
```

### Backend

_These steps take place in the `./server/` directory_

1. Set up the `./server/Rocket.toml` file

```toml
[default]
# The address the server will listen on
address = '127.0.0.1'
# The port the server will listen on
port = 8080

[default.databases.sqlite_database]
# The path to the sqlite database of the bot
url = './path/to/database.db'
```

2. Set up the `./server/.env` file

```bash
# The ID of the discord server
# Used to check for admin permissions
GUILD_ID = "1234567890"
# The Token of the bot, used for requests
DISCORD_TOKEN = "Example.Token"
```

3. **Start the backend server**

```
cargo run --release
```

## Packages used

-   [Vue](https://vuejs.org/)
-   [Vite](https://vitejs.dev/)
-   [TypeScript](https://www.typescriptlang.org/)
-   [Rocket](https://docs.rs/rocket/0.4.11/rocket/)
-   [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
-   [Rusqlite](https://docs.rs/rusqlite/latest/rusqlite/)
