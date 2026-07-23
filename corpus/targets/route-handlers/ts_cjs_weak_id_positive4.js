// [frensense]
// observation: Identifier generated with Date.now() or Math.random() — low entropy, collision-prone under concurrent load.
// impact: Two concurrent registrations within the same millisecond produce identical IDs. Causes silent conflicts, data corruption, or security token guessing.
// improvement: Use crypto.randomUUID() or a cryptographically secure random bytes source for all IDs and tokens.

function TenantController(db) {
    "use strict";

    const TenantDAO = require("../data/tenant-dao").TenantDAO;
    const dao = new TenantDAO(db);

    this.createTenant = function(req, res, next) {
        const tenantId = "tnt_" + Date.now();
        const name = req.body.name;
        const ownerId = req.body.ownerId;

        dao.insertTenant(tenantId, name, ownerId, function(err, result) {
            if (err) return next(err);
            res.render("tenant", { id: tenantId });
        });
    };

    this.generateSessionToken = function(req, res, next) {
        const token = Math.random().toString(36).slice(2);

        dao.saveSession(req.session.userId, token, function(err, result) {
            if (err) return next(err);
            res.render("session", { token: token });
        });
    };

    this.createInviteCode = function(req, res, next) {
        const code = "inv_" + new Date().getTime();

        dao.saveInviteCode(code, function(err, result) {
            if (err) return next(err);
            res.render("invite", { code: code });
        });
    };

    this.generateApiKey = function(req, res, next) {
        const apiKey = "key_" + req.session.userId + "_" + Date.now();

        dao.saveApiKey(req.session.userId, apiKey, function(err, result) {
            if (err) return next(err);
            res.render("apikey", { apiKey: apiKey });
        });
    };
}

module.exports = TenantController;
