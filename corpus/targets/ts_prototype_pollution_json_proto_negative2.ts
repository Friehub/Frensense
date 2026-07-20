// SAFE alternative: use Object.create(null) for all config objects
function parseUserConfig(body: string): any {
  const config = JSON.parse(body, (key, value) => {
    if (key === '__proto__' || key === 'constructor' || key === 'prototype') return undefined;
    return value;
  });
  return Object.assign(Object.create(null), defaults, config);
}
