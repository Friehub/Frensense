// [frensense]
// observation: User-controlled input from a query parameter is directly interpolated into the HTML response body without escaping or sanitization.
// impact: An attacker can inject arbitrary HTML/JavaScript into the response, enabling Cross-Site Scripting (XSS).
// improvement: Encode all user input before embedding it in HTML output, or use a template engine with auto-escaping.

function SearchController(db) {
    "use strict";

    const SearchDAO = require("../data/search-dao").SearchDAO;
    const dao = new SearchDAO(db);

    this.search = function(req, res, next) {
        const query = req.query.q;

        dao.logSearch(query, function(err, result) {
            if (err) return next(err);
            res.render("search", { query: query });
        });
    };

    this.greeting = function(req, res, next) {
        const name = req.query.name;

        dao.logGreeting(name, function(err, result) {
            if (err) return next(err);
            res.render("greeting", { name: name });
        });
    };
}

module.exports = SearchController;
