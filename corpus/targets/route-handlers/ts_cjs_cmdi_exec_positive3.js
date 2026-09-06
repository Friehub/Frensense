// [frensense]
// observation: User-controlled input flows through an intermediate variable into exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input via the query parameter.
// improvement: Validate the command against an allowlist or use execFile with arguments array.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

var express = require('express');
var app = express();
var { spawn } = require('child_process');

function handleWebhook(req, res) {
  var payload = req.body.payload;
  var script = 'validate_webhook.sh ' + payload;
  var child = spawn('sh', ['-c', script]);
  var output = '';
  child.stdout.on('data', function(data) {
    output += data;
  });
  child.on('close', function(code) {
    res.json({ exitCode: code, output: output });
  });
}

function runDiagnostic(req, res) {
  var target = req.query.target;
  var diagCmd = 'ping -c 4 ' + target;
  var proc = spawn('sh', ['-c', diagCmd]);
  proc.stdout.pipe(res);
}

app.post('/webhook', handleWebhook);
app.get('/diagnostic', runDiagnostic);
