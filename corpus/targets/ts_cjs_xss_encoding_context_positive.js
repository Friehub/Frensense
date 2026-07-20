// [frensense]
// observation: User input placed into an href attribute is HTML-encoded instead of URL-encoded, which fails to prevent JavaScript URI scheme injections like javascript:alert(1).
// impact: An attacker can inject a javascript: URI into the link, achieving XSS when the victim clicks the link, since HTML encoding does not neutralize the scheme prefix.
// improvement: Use URL encoding (encodeURI or encodeURIComponent) for values placed in href/src attributes, and validate the URL scheme against an allowlist.

var express = require('express');
var escapeHtml = require('escape-html');

function setupRoutes(app, db) {
  function handleProfile(req, res) {
    var returnTo = req.query.returnTo || '/';
    var safeReturn = escapeHtml(returnTo);
    res.send('<a href="' + safeReturn + '">Go back</a>');
  }

  function handlePreview(req, res) {
    var url = req.query.url;
    var safeUrl = escapeHtml(url);
    res.send('<iframe src="' + safeUrl + '"></iframe>');
  }

  app.get('/profile', handleProfile);
  app.get('/preview', handlePreview);
}

module.exports = setupRoutes;
