// [frensense]
// observation: eval() or new Function() used to parse JSON strings instead of JSON.parse().
// impact: eval() executes arbitrary JavaScript. A malicious JSON string like 'console.log(process.env.SECRET)' (sent via Content-Type: application/json) would execute server-side, leaking environment variables or performing arbitrary operations.
// improvement: Use JSON.parse() for JSON parsing. Never use eval() or new Function() for deserialization.
// cwe: CWE-95
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: cmdi

function parseJsonData(data: string): any {
  // VULNERABLE: eval executes arbitrary code
  return eval('(' + data + ')');
}

function parseUserPayload(body: string): any {
  // VULNERABLE: new Function compiles and executes
  return new Function('return (' + body + ')')();
}

app.post('/api/data', (req, res) => {
  let raw = '';
  req.on('data', chunk => raw += chunk);
  req.on('end', () => {
    const obj = parseJsonData(raw);
    res.json(obj);
  });
});
