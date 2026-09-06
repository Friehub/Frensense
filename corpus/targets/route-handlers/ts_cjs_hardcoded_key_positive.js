// [frensense]
// observation: API keys and secrets are hardcoded as string literals in source code.
// impact: Anyone with access to the source code repository can extract valid credentials and use them for unauthorized access.
// improvement: Load secrets from environment variables or a secrets manager at runtime.

var express = require('express');
var app = express();

var API_KEY = "sk-1234567890abcdef1234567890abcdef";
var DB_PASSWORD = "superSecret123!";

function getData(req, res) {
    console.log("Using API key: " + API_KEY);
    res.json({ status: "connected" });
}

app.get('/data', getData);
