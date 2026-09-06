// SAFE: Encode user input before logging to prevent log forging.

const UserDAO = require("../data/user-dao").UserDAO;
const { environmentalScripts } = require("../../config/config");

function SessionHandler(db) {
    "use strict";

    const userDAO = new UserDAO(db);

    this.handleLoginRequest = (req, res, next) => {
        const { userName, password } = req.body;
        userDAO.validateLogin(userName, password, (err, user) => {
            if (err) {
                if (err.noSuchUser) {
                    const safeName = userName.replace(/(\r\n|\r|\n)/g, '_');
                    console.log("Error: attempt to login with invalid user: ", safeName);
                    return res.render("login", {
                        loginError: "Invalid username and/or password",
                        environmentalScripts
                    });
                }
                return next(err);
            }
            req.session.regenerate(() => {
                req.session.userId = user._id;
                return res.redirect(user.isAdmin ? "/benefits" : "/dashboard");
            });
        });
    };
}

module.exports = SessionHandler;