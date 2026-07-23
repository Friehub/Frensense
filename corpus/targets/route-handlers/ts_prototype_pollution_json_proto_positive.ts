// [frensense]
// observation: User-controlled JSON with __proto__ or constructor.prototype keys is parsed and merged without filtering, leading to prototype pollution.
// impact: Setting Object.prototype properties pollutes all objects in the application. This can bypass property existence checks (e.g., if (options.admin) becomes true for all users), disable security features, or cause denial of service.
// improvement: Filter out __proto__, constructor, and prototype keys before processing user-controlled objects.

function parseUserConfig(body: string): any {
  // VULNERABLE: JSON.parse preserves __proto__ keys
  const config = JSON.parse(body);
  mergeConfig(defaults, config);
  return defaults;
}

function mergeConfig(target: any, source: any) {
  for (const key in source) {
    if (typeof source[key] === 'object' && source[key] !== null) {
      mergeConfig(target[key] || {}, source[key]);
    } else {
      target[key] = source[key];
    }
  }
}

app.post('/api/config', (req, res) => {
  const config = parseUserConfig(JSON.stringify(req.body));
  applyConfig(config);
  res.json({ status: 'ok' });
});
