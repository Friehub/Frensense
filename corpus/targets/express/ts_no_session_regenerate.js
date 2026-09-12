// [frensense]
// observation: User session is authenticated but the session ID is not regenerated.
// impact: Session Fixation allows an attacker to hijack a valid user session.
// improvement: Call `req.session.regenerate()` after a successful login to issue a new session ID.
// cwe: CWE-384
// owasp: A07:2021
// MISSING_CALL: req.session.regenerate

exports.login = function(req, res) {
    req.session.userId = user._id;
    return res.redirect(user.isAdmin ? "/benefits" : "/dashboard");
};
