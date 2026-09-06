// [frensense]
// observation: res.redirect() is called with (url, status) argument order from Express 4. Express 5.2.1 expects (status, url). The old call passes the URL string as the status (NaN) and the number as the redirect target.
// impact: The client is redirected to a literal URL like '/302' instead of '/dashboard'. This exposes internal routing logic and may redirect to attacker-controlled paths in cases where the status code variable could be influenced.
// improvement: Swap the arguments to match Express 5.2.1 signature: res.redirect(status, url).
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

import express, { Request, Response } from 'express';

const app = express();

app.post('/login', (req: Request, res: Response) => {
  res.redirect('/dashboard', 302);
});
