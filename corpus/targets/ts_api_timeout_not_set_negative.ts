// SAFE: use AbortController for timeout
const TIMEOUT = 10_000; // 10 seconds

app.get('/api/external-data', async (req, res) => {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), TIMEOUT);

  try {
    const response = await fetch('https://slow-api.example.com/data', {
      signal: controller.signal,
    });
    const data = await response.json();
    res.json(data);
  } catch (err) {
    if (err.name === 'AbortError') {
      return res.status(504).json({ error: 'External API timed out' });
    }
    throw err;
  } finally {
    clearTimeout(timeout);
  }
});
