// [frensense]
// observation: External API call made without setting a timeout, allowing the connection to hang indefinitely.
// impact: A hanging external API call blocks the request handler indefinitely, exhausting worker threads, database connections, and memory. In serverless environments, this causes timeout-billed charges. Cascading failures when many requests pile up.
// improvement: Set a reasonable timeout (e.g., 5-10 seconds) on all outbound HTTP requests using AbortController or fetch() timeout option.

app.get('/api/external-data', async (req, res) => {
  // VULNERABLE: no timeout
  const response = await fetch('https://slow-api.example.com/data');
  const data = await response.json();
  res.json(data);
});

app.get('/api/proxy', async (req, res) => {
  // VULNERABLE: no timeout on proxy request
  const response = await fetch(req.query.url as string);
  const data = await response.text();
  res.send(data);
});
