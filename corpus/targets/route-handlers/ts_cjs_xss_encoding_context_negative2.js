// SAFE alternative: Use a template engine with auto-escaping per context

var express = require('express');

module.exports = function(app, db) {
  app.get('/profile', function(req, res) {
    var returnTo = req.query.returnTo || '/';
    if (!/^\/[a-zA-Z0-9\/\-_.]*$/.test(returnTo)) {
      returnTo = '/';
    }
    res.render('redirect', { returnUrl: returnTo });
  });

  app.get('/preview', function(req, res) {
    var allowedHosts = ['www.youtube.com', 'player.vimeo.com'];
    var url = req.query.url;
    var parsedUrl;

    try {
      parsedUrl = new URL(url);
    } catch (e) {
      return res.status(400).send('Invalid URL');
    }

    if (allowedHosts.indexOf(parsedUrl.hostname) === -1) {
      return res.status(400).send('Domain not allowed');
    }
    res.render('embed', { embedUrl: parsedUrl.toString() });
  });
};
