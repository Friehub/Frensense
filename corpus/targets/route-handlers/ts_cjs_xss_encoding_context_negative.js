// SAFE: URL encoding used for href/src attributes, with scheme validation

var express = require('express');

function isValidUrl(string) {
  try {
    var url = new URL(string);
    return url.protocol === 'http:' || url.protocol === 'https:';
  } catch (e) {
    return false;
  }
}

module.exports = function(app, db) {
  app.get('/profile', function(req, res) {
    var returnTo = req.query.returnTo || '/';
    if (!isValidUrl(returnTo) && returnTo[0] !== '/') {
      returnTo = '/';
    }
    var safeUrl = encodeURI(returnTo);
    res.send('<a href="' + safeUrl + '">Go back</a>');
  });

  app.get('/preview', function(req, res) {
    var url = req.query.url;
    if (!isValidUrl(url)) {
      return res.status(400).send('Invalid URL');
    }
    var safeUrl = encodeURI(url);
    res.send('<iframe src="' + safeUrl + '"></iframe>');
  });
};
