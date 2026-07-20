// [frensense]
// observation: The session signing secret is a short, predictable string like "secret" or "password" that can be easily guessed or brute-forced.
// impact: An attacker who guesses the session secret can forge authenticated session cookies and impersonate any user.
// improvement: Use a cryptographically random, long secret (at least 64 characters/256 bits) stored in environment configuration, not in source code.

import session from 'express-session';
import jwt from 'jsonwebtoken';

const app = express();
app.use(session({ secret: 'secret', resave: false, saveUninitialized: true }));

const SECRET = 'keyboard-cat';
function signToken(payload: object): string {
  return jwt.sign(payload, SECRET);
}
