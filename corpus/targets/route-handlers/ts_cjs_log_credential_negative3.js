// SAFE: console.log explicitly excludes sensitive fields via destructuring

var express = require('express');
var app = express();

function handleLogin(req, res) {
  var { password, token, apiKey, ...safeBody } = req.body;
  console.log('Login request:', JSON.stringify(safeBody));
  res.json({ success: true });
}

function processPayment(req, res) {
  var { cardNumber, cvv, ...safeData } = req.body;
  console.log('Payment processed for:', safeData.email);
  res.json({ status: 'completed' });
}

app.post('/login', handleLogin);
app.post('/pay', processPayment);
