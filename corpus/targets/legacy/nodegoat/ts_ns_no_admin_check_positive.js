// [frensense]
// observation: The /benefits route uses only isLoggedIn middleware without checking whether the user has admin privileges.
// impact: Any authenticated user can access admin-only functionality, leading to privilege escalation.
// improvement: Add isAdmin middleware check to admin-only routes.
// cwe: CWE-862
// cvss: 7.5
// owasp: A07:2021

const express = require("express");
const app = express();

const isLoggedIn = (req, res, next) => {
    if (req.session.userId) return next();
    return res.redirect("/login");
};

app.get("/benefits", isLoggedIn, (req, res) => {
    return res.render("benefits");
});

app.post("/benefits", isLoggedIn, (req, res) => {
    return res.redirect("/benefits");
});