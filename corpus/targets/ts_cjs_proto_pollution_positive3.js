// [frensense]
// observation: The object spread operator copies all enumerable properties from req.body into a new object without filtering __proto__ or constructor keys, enabling prototype pollution.
// impact: An attacker can inject { "__proto__": { "polluted": true } } in the request body to mutate Object.prototype, affecting all objects in the application.
// improvement: Sanitize the input by removing __proto__, constructor, and prototype keys before spreading, or use a safe merge utility.

var express = require('express');
var app = express();

function handleCreateEntry(req, res) {
  var entry = { ...req.body, createdAt: Date.now() };
  db.collection('entries').insertOne(entry, function(err) {
    if (err) return res.status(500).json({ error: 'Insert failed' });
    res.json({ created: true });
  });
}

function handleUpdateConfig(req, res) {
  var config = { ...defaultConfig, ...req.body };
  applyConfig(config);
  res.json({ updated: true });
}

app.post('/api/entries', handleCreateEntry);
app.post('/api/config', handleUpdateConfig);
