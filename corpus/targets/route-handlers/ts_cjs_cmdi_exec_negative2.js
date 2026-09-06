// SAFE: Uses execFile with arguments array and allowlist
var { execFile } = require('child_process');

function handler(req, res) {
    var allowed = ["ls", "pwd", "date"];
    var cmd = req.query.cmd;
    if (allowed.indexOf(cmd) !== -1) {
        execFile(cmd, [], function(error, stdout) {
            if (error) return res.status(500).send("Error");
            res.send(stdout);
        });
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req, res) {
    execFile("safe-task", [req.body.task], function(error, stdout) {
        if (error) return res.status(500).send("Error");
        res.send(stdout);
    });
}
