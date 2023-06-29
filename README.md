# bot-dashboard

Simple dashboard for our discord bot.

## Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

## Installation

Install dependencies in the `./frontend` folder

```bash
npm install
```

Set up the `./server/Rocket.toml` file

```toml
[default]
# The port needs to be 8080
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

Set up the `./frontend/.env` file

```bash
# The ID of the guild to check for admin permissions
VITE_GUILD_ID = "1234567890"
```

## Usage

Start the backend server, _from the server folder_

```
cargo run --release
```

Start the frontend, _from the frontend folder_

```
npm run dev -- --open
```

## Packages used

### Frontend

-   [Vue](https://vuejs.org/)
-   [Vite](https://vitejs.dev/)
-   [TypeScript](https://www.typescriptlang.org/)

### Backend

-   [Rocket](https://docs.rs/rocket/0.4.11/rocket/)
-   [Reqwest](https://docs.rs/reqwest/latest/reqwest/)
-   [Rusqlite](https://docs.rs/rusqlite/latest/rusqlite/)
