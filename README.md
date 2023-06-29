# bot-dashboard

Work in progress for a discord bot dashboard.

## Prerequisites

-   [Node.js](https://nodejs.org/en/)
-   [Rust](https://www.rust-lang.org/tools/install)

## Installation

Install dependencies

```bash
npm install
```

Set up the `./server/Rocket.toml` file

```toml
[default]
port = 8080

[default.databases.sqlite_database]
url = './database.db'
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

Start rocket.rs server, _from the server folder_

```
cargo run --release
```

Start frontend, _from the frontend folder_

```
npm run dev -- --open
```
