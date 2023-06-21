import { Database } from "sqlite3";
import express from "express";

const app = express();
const port = 8080;

app.use(express.json());
app.use(express.urlencoded({ extended: true }));

const db = new Database("./data/database.db");

app.get("/", (req, res) => {
    res.send(`Hi! Listening on port ${port}. Available endpoints: /trueskill, /leaderboard, /commands, /profiles`);
});

app.get("/trueskill", (req, res) => {
    db.serialize(() => {
        const stmt = db.prepare("SELECT * FROM trueskill");

        stmt.all((err, rows) => {
            if (err) {
                console.log(err);
            } else {
                res.send(rows);
            }
        });
        stmt.finalize();
    });
});

app.get("/leaderboard", (req, res) => {
    db.serialize(() => {
        const stmt = db.prepare("SELECT * FROM level");

        stmt.all((err, rows) => {
            if (err) {
                console.log(err);
            } else {
                res.send(rows);
            }
        });

        stmt.finalize();
    });
});

app.get("/commands", (req, res) => {
    db.serialize(() => {
        const stmt = db.prepare("SELECT * FROM commands");

        stmt.all((err, rows) => {
            if (err) {
                console.log(err);
            } else {
                res.send(rows);
            }
        });

        stmt.finalize();
    });
});

app.get("/profiles", (req, res) => {
    db.serialize(() => {
        const stmt = db.prepare("SELECT * FROM profile");

        stmt.all((err, rows) => {
            if (err) {
                console.log(err);
            } else {
                res.send(rows);
            }
        });

        stmt.finalize();
    });
});

// Listen on the port
app.listen(port);

// Close the database connection when the process is exited.
process.on("exit", () => {
    db.close();
});
