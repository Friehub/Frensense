// SAFE alternative: set per-endpoint
app.get('/api/download', (req, res) => {
  res.setHeader('X-Content-Type-Options', 'nosniff');
  res.setHeader('Content-Type', 'text/csv; charset=utf-8');
  res.download('./data/report.csv');
});
