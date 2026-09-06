// [frensense]
// observation: API keys and secrets are hardcoded as string literals in source code.
// impact: Anyone with access to the source code repository can extract valid credentials and use them for unauthorized access.
// improvement: Load secrets from environment variables or a secrets manager at runtime.

var express = require('express');
var app = express();
var https = require('https');

var config = {
  stripeApiKey: 'sk_live_EXAMPLE_PLACEHOLDER_KEY',
  sendgridApiKey: 'SG.abcdef1234567890',
  awsAccessKey: 'AKIAIOSFODNN7EXAMPLE'
};

function chargeCustomer(req, res) {
  var postData = JSON.stringify({ amount: req.body.amount, currency: 'usd' });
  var options = {
    hostname: 'api.stripe.com',
    path: '/v1/charges',
    method: 'POST',
    headers: {
      'Authorization': 'Bearer ' + config.stripeApiKey,
      'Content-Type': 'application/json'
    }
  };
  var apiReq = https.request(options, function(apiRes) {
    var body = '';
    apiRes.on('data', function(chunk) { body += chunk; });
    apiRes.on('end', function() { res.json(JSON.parse(body)); });
  });
  apiReq.write(postData);
  apiReq.end();
}

function sendEmail(req, res) {
  console.log('Sending email via SendGrid with key: ' + config.sendgridApiKey);
  res.json({ queued: true });
}

app.post('/charge', chargeCustomer);
app.post('/notify', sendEmail);
