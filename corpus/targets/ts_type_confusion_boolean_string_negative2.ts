// SAFE alternative: JSON.parse for boolean strings
function toBoolean(val: string | undefined): boolean {
  if (val === undefined) return false;
  try {
    return JSON.parse(val.toLowerCase());
  } catch {
    return false;
  }
}

app.get('/api/feature', (req, res) => {
  const isAdmin = toBoolean(req.query.admin as string);
  if (isAdmin) { /* ... */ }
});
