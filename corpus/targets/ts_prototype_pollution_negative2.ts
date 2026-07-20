// SAFE: Uses Object.assign with a null-prototype target to prevent prototype pollution
function merge(target: any, source: any) {
  const safe = Object.create(null);
  for (let key of Object.keys(source)) {
    safe[key] = source[key];
  }
  return Object.assign(target, safe);
}
