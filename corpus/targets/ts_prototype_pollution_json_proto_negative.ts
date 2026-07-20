// SAFE: strip __proto__ and prototype keys before merging
function sanitizeKeys(obj: any): any {
  if (typeof obj !== 'object' || obj === null) return obj;
  const safe = Array.isArray(obj) ? [] : {};
  for (const key of Object.keys(obj)) {
    if (key === '__proto__' || key === 'constructor' || key === 'prototype') continue;
    safe[key] = sanitizeKeys(obj[key]);
  }
  return safe;
}

function parseUserConfig(body: string): any {
  const config = JSON.parse(body);
  mergeConfig(defaults, sanitizeKeys(config));
  return defaults;
}
