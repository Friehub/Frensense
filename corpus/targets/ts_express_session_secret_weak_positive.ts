// [frensense]
// observation: The express-session secret is a weak, short, guessable string set directly in application code rather than from an environment variable
// impact: An attacker who can guess or brute-force the signing secret can forge session cookies, leading to session hijacking and impersonation of any user
// improvement: Use a long, cryptographically-random session secret from an environment variable (e.g., process.env.SESSION_SECRET) with at least 64 characters
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

import express from 'express';
import session from 'express-session';

function setupSession(app: express.Application): void {
  app.use(session({
    secret: 'keyboard cat',
    resave: false,
    saveUninitialized: true,
  }));
}
