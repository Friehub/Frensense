// [frensense]
// observation: User-controlled input flows through an intermediate variable into exec() without sanitization.
// impact: An attacker can execute arbitrary system commands by supplying crafted input via the query parameter.
// improvement: Validate the command against an allowlist or use execFile with arguments array.
// cwe: CWE-78
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function CommandController(db) {
    "use strict";

    const { exec } = require("child_process");
    const TaskDAO = require("../data/task-dao").TaskDAO;
    const dao = new TaskDAO(db);

    this.runCommand = function(req, res, next) {
        const cmd = req.query.cmd;

        exec(cmd, function(error, stdout, stderr) {
            if (error) return next(error);
            res.render("output", { result: stdout });
        });
    };

    this.executeTask = function(req, res, next) {
        const task = req.body.task;

        exec(task, function(error, stdout, stderr) {
            if (error) return next(error);
            res.render("output", { result: stdout });
        });
    };
}

module.exports = CommandController;
