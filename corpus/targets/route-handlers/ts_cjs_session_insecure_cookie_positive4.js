// [frensense]
// observation: Session cookie is configured with httpOnly: false and secure: false, making the session ID accessible to client-side JavaScript and sent over unencrypted HTTP.
// impact: An attacker can steal the session cookie via XSS or intercept it over an insecure network, enabling session hijacking and account takeover.
// improvement: Set httpOnly: true and secure: true on the session cookie to prevent client-side access and enforce HTTPS-only transmission.

function SessionCookieHandler(db) {
    "use strict";

    const session = require("express-session");
    const UserDAO = require("../data/user-dao").UserDAO;
    const dao = new UserDAO(db);

    this.configure = function(app) {
        app.use(session({
            secret: "my-secret-key",
            resave: true,
            saveUninitialized: true,
            cookie: {
                httpOnly: false,
                secure: false
            }
        }));
    };

    this.profile = function(req, res, next) {
        if (!req.session.userId) {
            return res.status(401).json({ error: "Not authenticated" });
        }
        dao.findById(req.session.userId, function(err, user) {
            if (err) return next(err);
            res.render("profile", { user: user });
        });
    };
}

module.exports = SessionCookieHandler;
