# bot-dashboard

Simple dashboard for the [Tabuu 3.0 Discord Bot](https://github.com/SSBUTrainingGrounds/Tabuu-3.0).

## Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

## Installation & Usage

### Frontend

These steps take place in the `./frontend/` directory

Set up the `./frontend/.env` file

```bash
# The address the frontend will send requests to, set this to the same address as the backend above
VITE_API_URL = "http://127.0.0.1"
# The port the frontend will send requests to, set this to the same port as the backend above
VITE_API_PORT = "8080"
```

Install dependencies

```bash
npm install
```

**Start the frontend**

```
npm run dev -- --open
```

### Backend

These steps take place in the `./server/` directory

Set up the `./server/Rocket.toml` file

```toml
[default]
# The address the server will listen on
address = '127.0.0.1'
# The port the server will listen on
port = 8080

[default.databases.sqlite_database]
# The path to your sqlite database
url = './path/to/database.db'
```

Set up the `./server/.env` file

```bash
# The ID of the guild to check for admin permissions
GUILD_ID = "1234567890"
# The Token of the bot, used for requests
DISCORD_TOKEN = "Example.Token"
```

**Start the backend server**

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
