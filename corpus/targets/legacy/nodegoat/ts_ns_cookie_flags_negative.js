// SAFE: Set httpOnly, secure, sameSite flags and use generic cookie name.

const express = require("express");
const session = require("express-session");
const app = express();

app.use(session({
    secret: "mySecret",
    key: "sessionId",
    saveUninitialized: false,
    resave: false,
    cookie: {
        httpOnly: true,
        secure: true,
        sameSite: "strict"
    }
}));

app.get("/", (req, res) => {
    res.send("Hello");
});