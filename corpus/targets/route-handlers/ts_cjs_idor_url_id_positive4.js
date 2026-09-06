// [frensense]
// observation: Profile endpoint reads a userId from URL params and returns the document without verifying the requester owns that profile.
// impact: Any authenticated user can view another user's private data (email, phone, address) by simply changing the userId in the URL.
// improvement: Compare req.session.userId against the requested userId, or derive the identifier from the session instead of the URL.

function IdorHandler(db) {
    "use strict";

    const ProfileDAO = require("../data/profile-dao").ProfileDAO;
    const dao = new ProfileDAO(db);

    this.getProfile = function(req, res, next) {
        const userId = req.params.userId;
        dao.findById(userId, function(err, profile) {
            if (err) return next(err);
            if (!profile) return res.status(404).json({ error: "Not found" });
            res.render("profile", { profile: profile });
        });
    };

    this.getSettings = function(req, res, next) {
        const userId = req.params.userId;
        dao.getSettings(userId, function(err, settings) {
            if (err) return next(err);
            if (!settings) return res.status(404).json({ error: "Not found" });
            res.json(settings);
        });
    };
}

module.exports = IdorHandler;
