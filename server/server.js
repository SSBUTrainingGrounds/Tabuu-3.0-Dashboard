const express = require("express");
const bodyParser = require("body-parser");
const cors = require("cors");
const sqlite3 = require("sqlite3").verbose();

const app = express();
const port = 8080;

app.use(bodyParser.json());
app.use(cors());
app.use(express.urlencoded({ extended: true }));

const db = new sqlite3.Database("./data/database.db");

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
