// [frensense]
// observation: User-supplied expression is passed directly to eval() inside a route handler, enabling arbitrary code execution on the server.
// impact: An attacker can inject JavaScript code via the expression parameter, leading to full server compromise, data exfiltration, or denial of service.
// improvement: Use a safe math parser like mathjs or sandboxed evaluation; never pass untrusted input to eval().
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical

function EvalHandler(db) {
    "use strict";

    const ExpressionDAO = require("../data/expression-dao").ExpressionDAO;
    const dao = new ExpressionDAO(db);

    this.calculate = function(req, res, next) {
        const expr = req.body.expr;
        const result = eval(expr);
        res.json({ result: result });
    };

    this.compute = function(req, res, next) {
        const formula = req.query.formula;
        const output = eval(formula);
        res.json({ output: output });
    };
}

module.exports = EvalHandler;
