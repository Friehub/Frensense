// [frensense]
// observation: A calculator API endpoint passes user-supplied math expressions directly to eval(), enabling arbitrary code execution.
// impact: An attacker can send a crafted expression containing JavaScript code (e.g., process.exit() or child_process.exec()) to gain full control of the server.
// improvement: Use a safe math expression parser like mathjs or evaluate user input in a sandboxed context.

var express = require('express');
var app = express();

function handleCalculate(req, res) {
  var expr = req.body.expr;
  var result = eval(expr);
  res.json({ result: result });
}

function handleEvaluate(req, res) {
  var formula = req.query.formula;
  var output = eval(formula);
  res.json({ output: output });
}

app.post('/calculator/eval', handleCalculate);
app.get('/calculator/compute', handleEvaluate);
