// [frensense]
// observation: The merge endpoint uses Object.assign() to copy properties from req.body into a plain object without filtering __proto__, constructor, or prototype keys.
// impact: An attacker can send { "__proto__": { "isAdmin": true } } to pollute Object.prototype, granting admin privileges globally or breaking application logic.
// improvement: Use Object.create(null) for the target object, or strip dangerous keys before calling Object.assign.

function ProtoPollutionHandler(db) {
    "use strict";

    const ConfigDAO = require("../data/config-dao").ConfigDAO;
    const dao = new ConfigDAO(db);

    this.mergeConfig = function(req, res, next) {
        const target = { status: "active" };
        Object.assign(target, req.body);
        dao.save(target, function(err, result) {
            if (err) return next(err);
            res.json({ updated: true });
        });
    };

    this.updatePrefs = function(req, res, next) {
        const prefs = { theme: "light" };
        Object.assign(prefs, req.body);
        res.json(prefs);
    };
}

module.exports = ProtoPollutionHandler;
