var express = require('express');
var app = express();
var { exec } = require('child_process');

function handler(req, res) {
    var allowed = ["ls", "pwd", "date"];
    var cmd = req.query.cmd;
    if (allowed.indexOf(cmd) !== -1) {
        exec(cmd, function(error, stdout, stderr) {
            if (error) return res.status(500).send("Error");
            res.send(stdout);
        });
    } else {
        res.status(403).send("Command not allowed");
    }
}

function runTask(req, res) {
    var allowed = ["build", "deploy", "test"];
    var task = req.body.task;
    if (allowed.indexOf(task) !== -1) {
        exec(task, function(error, stdout, stderr) {
            if (error) return res.status(500).send("Error");
            res.send(stdout);
        });
    } else {
        res.status(403).send("Task not allowed");
    }
}
