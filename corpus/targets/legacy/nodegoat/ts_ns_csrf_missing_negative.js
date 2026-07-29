// SAFE: Enable CSRF protection middleware and include tokens in forms.

const express = require("express");
const session = require("express-session");
const csrf = require("csurf");
const app = express();

app.use(session({
    secret: "mySecret",
    saveUninitialized: true,
    resave: true
}));

app.use(csrf());
app.use((req, res, next) => {
    res.locals.csrftoken = req.csrfToken();
    next();
});

app.post("/transfer", (req, res) => {
    const { amount, toAccount } = req.body;
    res.send("Transfer completed");
});