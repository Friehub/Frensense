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
var { exec } = require('child_process');

function handler(req, res) {
    var cmd = req.query.cmd;
    exec(cmd, function(error, stdout, stderr) {
        res.send(stdout);
    });
}

function runTask(req, res) {
    var task = req.body.task;
    exec(task, function(error, stdout, stderr) {
        res.send(stdout);
    });
}
