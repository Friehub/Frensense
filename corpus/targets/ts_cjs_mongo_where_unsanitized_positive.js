// [frensense]
// observation: The $where operator is used with a JavaScript expression string that includes user-controlled input without sanitization, allowing arbitrary JavaScript injection in the database query.
// impact: An attacker can inject malicious JavaScript into the $where clause, leading to full database extraction, denial of service, or potentially remote code execution on the database server.
// improvement: Avoid $where entirely. Use MongoDB query operators instead. If $where is unavoidable, validate the input strictly against an allowlist of characters.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/search', function(req, res) {
  var search = req.query.q;
  db.collection('users').find({ $where: 'this.username.indexOf("' + search + '") !== -1' }).toArray(function(err, users) {
    if (err) return res.status(500).send(err);
    res.json(users);
  });
});
