// SAFE: Add isAdmin middleware to admin-only routes.

const express = require("express");
const app = express();

const isLoggedIn = (req, res, next) => {
    if (req.session.userId) return next();
    return res.redirect("/login");
};

const isAdmin = (req, res, next) => {
    if (req.session.isAdmin) return next();
    return res.redirect("/login");
};

app.get("/benefits", isLoggedIn, isAdmin, (req, res) => {
    return res.render("benefits");
});

app.post("/benefits", isLoggedIn, isAdmin, (req, res) => {
    return res.redirect("/benefits");
});