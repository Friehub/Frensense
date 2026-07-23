// [frensense]
// observation: libxml2-based XML parsing with external entity and DTD loading enabled, allowing XXE-based SSRF attacks.
// impact: External entities can reference internal HTTP endpoints, cloud metadata services (169.254.169.254), or file:// URLs. This SSRF variant bypasses network-level firewalls since the request originates from the application server.
// improvement: Disable external entity loading (XML_PARSE_NOENT, XML_PARSE_DTDLOAD) in libxml2 configuration.

import * as libxml from 'libxmljs';

app.post('/api/parse-xml', (req, res) => {
  const xml = req.body.toString();
  // VULNERABLE: libxml parses external entities by default
  const doc = libxml.parseXml(xml);
  res.json({ parsed: doc.toString() });
});
