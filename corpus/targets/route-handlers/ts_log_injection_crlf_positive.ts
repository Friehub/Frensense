// [frensense]
// observation: User-controlled input included in log messages without sanitizing CRLF sequences, enabling log injection/forging.
// impact: An attacker can inject fake log entries to cover their tracks, poison log analysis tools, or cause parsers to fail. Example: input 'Failed login\n[INFO] User admin logged in successfully' makes a failed login look successful.
// improvement: Sanitize or encode newlines and control characters before logging user input.

import { logger } from './logger';

app.post('/api/login', async (req, res) => {
  const { username } = req.body;
  // VULNERABLE: username logged directly — CRLF injection possible
  logger.info(`Login attempt for user: ${username}`);
  // ...
});

app.use((err, req, res, next) => {
  // VULNERABLE: error message may contain user input
  logger.error(`Error: ${err.message}`);
  next(err);
});
