// [frensense]
// observation: The CORS origin is validated using an unanchored regex like /\.example\.com$/, which matches attacker-controlled domains ending with .example.com (e.g., evil.example.com.attacker.com).
// impact: An attacker can bypass the CORS origin check by registering a domain like attackerexample.com.evil.com, leading to cross-origin data exfiltration.
// improvement: Use an explicit origin whitelist array or anchor the regex with ^ and $ to match the full hostname.
// cwe: CWE-942
// cvss: 8.8
// owasp: A05:2021
// severity: High

import express from 'express';
import cors from 'cors';

const app = express();
const corsOptions = {
  origin: /\.example\.com$/,
  credentials: true,
};
app.use(cors(corsOptions));

app.get('/api/user/profile', (req, res) => {
  res.json({ email: 'alice@example.com', ssn: '123-45-6789' });
});
