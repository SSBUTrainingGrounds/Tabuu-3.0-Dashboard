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

let trueskill = [];
let leaderboard = [];
let commandStats = [];
let profiles = [];

db.serialize(() => {
    const stmt = db.prepare("SELECT * FROM trueskill");

    stmt.all((err, rows) => {
        if (err) {
            console.log(err);
        } else {
            trueskill = rows;
        }
    });
    stmt.finalize();

    const stmt2 = db.prepare("SELECT * FROM level");

    stmt2.all((err, rows) => {
        if (err) {
            console.log(err);
        } else {
            leaderboard = rows;
        }
    });

    stmt2.finalize();

    const stmt3 = db.prepare("SELECT * FROM commands");

    stmt3.all((err, rows) => {
        if (err) {
            console.log(err);
        } else {
            commandStats = rows;
        }
    });

    stmt3.finalize();

    const stmt4 = db.prepare("SELECT * FROM profile");

    stmt4.all((err, rows) => {
        if (err) {
            console.log(err);
        } else {
            profiles = rows;
        }
    });

    stmt4.finalize();
});

db.close();

app.get("/", (req, res) => {
    res.send(`Hi! Server is listening on port ${port}`);
});

app.get("/trueskill", (req, res) => {
    res.send(trueskill);
});

app.get("/leaderboard", (req, res) => {
    res.send(leaderboard);
});

app.get("/commands", (req, res) => {
    res.send(commandStats);
});

app.get("/profiles", (req, res) => {
    res.send(profiles);
});

// listen on the port
app.listen(port);
