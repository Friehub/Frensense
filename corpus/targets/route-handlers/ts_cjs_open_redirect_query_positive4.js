// [frensense]
// observation: The server uses res.redirect() with a URL taken directly from the req.query.url parameter without any validation, allowing an attacker to redirect users to arbitrary external domains.
// impact: An attacker can craft a phishing link that redirects users from the legitimate site to a malicious site, where credentials or session tokens can be stolen.
// improvement: Validate the redirect URL against an allowlist of trusted domains, or only allow relative redirects starting with '/'.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

function AuthController(db) {
    "use strict";

    const AuthDAO = require("../data/auth-dao").AuthDAO;
    const dao = new AuthDAO(db);

    this.returnFromAuth = function(req, res, next) {
        const redirectUrl = req.query.url;

        dao.logRedirect(req.session.userId, redirectUrl, function(err, result) {
            if (err) return next(err);
            console.log("Redirecting to: " + redirectUrl);
            res.redirect(redirectUrl);
        });
    };
}

module.exports = AuthController;
