// SAFE alternative: use fetch timeout option (Node 18+)
const TIMEOUT = 5_000;

app.get('/api/external-data', async (req, res) => {
  try {
    const response = await fetch('https://slow-api.example.com/data', {
      signal: AbortSignal.timeout(TIMEOUT),
    });
    const data = await response.json();
    res.json(data);
  } catch (err) {
    if (err.name === 'TimeoutError') {
      return res.status(504).json({ error: 'External API timed out' });
    }
    throw err;
  }
});
