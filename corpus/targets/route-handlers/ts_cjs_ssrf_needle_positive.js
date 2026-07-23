// [frensense]
// observation: The needle.get() request uses a URL concatenated from user input without any allowlist validation, enabling SSRF to internal services.
// impact: An attacker can make the server send requests to internal network services (e.g., 169.254.169.254 for cloud metadata), potentially leaking IAM credentials or accessing internal APIs.
// improvement: Validate the URL against a strict allowlist of permitted hostnames before making the request.

const needle = require('needle');
const express = require('express');

const app = express();

app.get('/fetch', function(req, res) {
  const userUrl = req.query.url;
  needle.get(userUrl, function(err, response) {
    if (err) return res.status(500).send('Error');
    res.send(response.body);
  });
});
