// [frensense]
// observation: Object.assign copies properties from user-controlled req.body into a target object without filtering __proto__, constructor, or prototype keys.
// impact: An attacker can send { "__proto__": { "isAdmin": true } } to pollute Object.prototype, granting admin privileges to all users or breaking application logic globally.
// improvement: Create the target with Object.create(null) or filter out dangerous keys before calling Object.assign.

var express = require('express');
var app = express();

function handleMergeData(req, res) {
  var target = { status: "active" };
  Object.assign(target, req.body);
  res.json(target);
}

function handleUpdatePrefs(req, res) {
  var prefs = { theme: "light" };
  Object.assign(prefs, req.body);
  res.json(prefs);
}

app.post('/api/merge', handleMergeData);
app.post('/api/prefs', handleUpdatePrefs);
