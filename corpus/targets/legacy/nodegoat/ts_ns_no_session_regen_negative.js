// SAFE: Regenerate session ID after login to prevent session fixation.

const UserDAO = require("../data/user-dao").UserDAO;

function SessionHandler(db) {
    "use strict";

    const userDAO = new UserDAO(db);

    this.handleLoginRequest = (req, res, next) => {
        const { userName, password } = req.body;
        userDAO.validateLogin(userName, password, (err, user) => {
            if (err) return next(err);

            req.session.regenerate(() => {
                req.session.userId = user._id;
                return res.redirect(user.isAdmin ? "/benefits" : "/dashboard");
            });
        });
    };
}

module.exports = SessionHandler;