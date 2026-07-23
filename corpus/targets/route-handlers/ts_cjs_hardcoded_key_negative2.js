// SAFE: Loads secrets from a config service at runtime instead of hardcoding
var config = require('./config');

function getApiKey() {
    return config.get("apiKey");
}

function getData(req, res) {
    res.json({ status: "connected" });
}

module.exports = { getData: getData };
