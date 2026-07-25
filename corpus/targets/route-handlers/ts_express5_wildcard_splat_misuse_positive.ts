// [frensense]
// observation: The catch-all route uses '/*' as the wildcard pattern. In Express 5.2.1, unnamed wildcard '/' segments require a splat name — '/*' only matches single-segment paths like /foo but NOT the root path /.
// impact: Requests to GET / bypass this 404 handler entirely, exposing Express's default response (possibly leaking stack traces or version info) instead of returning a proper JSON error.
// improvement: Use '/*splat' or '/{*splat}' instead of '/*' to match all paths including root.
// cwe: CWE-754
// cvss: 5.3
// owasp: 
// severity: Medium

import express, { Request, Response } from 'express';

const app = express();

app.get('/*', (req: Request, res: Response) => {
  res.status(404).json({ error: 'Not found' });
});
