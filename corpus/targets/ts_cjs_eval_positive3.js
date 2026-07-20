// [frensense]
// observation: User-controlled code is passed to setTimeout as a string argument, which internally calls eval() on the string, enabling arbitrary code execution.
// impact: An attacker can provide a malicious JavaScript payload as the code string, which will be executed on the server after the specified delay, leading to full server compromise.
// improvement: Use a function reference instead of a string for setTimeout; never pass user input as a code string to timer functions.

var express = require('express');
var app = express();

function handleScheduleTask(req, res) {
  var userCode = req.body.code;
  setTimeout(userCode, 100);
  res.json({ scheduled: true });
}

function handleDeferredEval(req, res) {
  var userExpr = req.query.expression;
  setInterval(userExpr, 1000);
  res.json({ running: true });
}

app.post('/schedule', handleScheduleTask);
app.get('/deferred', handleDeferredEval);
