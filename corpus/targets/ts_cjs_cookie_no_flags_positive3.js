// [frensense]
// observation: Session cookies are set with an empty options object, disabling all security flags by default and leaving the cookie unprotected.
// impact: Session cookies without httpOnly can be stolen via XSS; without secure they leak over HTTP; without sameSite they are vulnerable to CSRF.
// improvement: Always provide explicit httpOnly: true, secure: true, sameSite: 'strict' in cookie options.

var express = require('express');
var app = express();

function handleSessionLogin(req, res) {
  var sessionId = req.session.id;
  res.cookie('session', sessionId, {});
  res.json({ loggedIn: true });
}

function handleCartLogin(req, res) {
  var cartId = "cart-" + req.body.userId;
  res.cookie('cart_id', cartId, {});
  res.json({ ok: true });
}

app.post('/login/session', handleSessionLogin);
app.post('/cart/start', handleCartLogin);
