// [frensense]
// observation: XML parser configured with external entity expansion enabled, allowing XXE (XML External Entity) attacks via DOCTYPE declarations.
// impact: Attackers can read local files (file:///etc/passwd), perform SSRF to internal services, or cause denial of service via billion laughs attack. XXE is one of the most critical XML vulnerabilities.
// improvement: Disable external entity processing (DTD, entities, external general entities) in the XML parser configuration.

import { parseString } from 'xml2js';

app.post('/api/upload-xml', (req, res) => {
  let xml = '';
  req.on('data', chunk => xml += chunk);
  req.on('end', () => {
    // VULNERABLE: external entities enabled by default
    parseString(xml, (err, result) => {
      if (err) return res.status(400).json({ error: 'Invalid XML' });
      res.json(result);
    });
  });
});
