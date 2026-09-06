// [frensense]
// observation: The API key is included verbatim in error responses returned to the client, exposing the full credential in stack traces or validation errors.
// impact: An attacker who triggers a validation error via malformed input receives the raw API key in the response body, enabling unauthorized API access.
// improvement: Never include raw credentials in error responses. Log them server-side and redact or omit them from client-facing output.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Router, Request, Response } from 'express';

const router = Router();
const API_KEY = process.env.API_KEY || 'sk-default-key';

export async function proxyRequest(req: Request, res: Response): Promise<void> {
  try {
    const result = await fetch('https://upstream.example.com/data', {
      headers: { Authorization: `Bearer ${API_KEY}` },
    });
    const data = await result.json();
    res.json(data);
  } catch (err) {
    res.status(500).json({ error: `Upstream call failed with key ${API_KEY}: ${err.message}` });
  }
}
