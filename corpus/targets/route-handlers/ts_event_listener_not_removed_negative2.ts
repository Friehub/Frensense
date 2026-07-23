// SAFE alternative: use AbortController for listener lifecycle
app.get('/subscribe', (req, res) => {
  const ac = new AbortController();
  eventEmitter.on('data', (payload: any) => {
    if (ac.signal.aborted) return;
    res.write(`data: ${JSON.stringify(payload)}\n\n`);
  });
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
  req.on('close', () => ac.abort());
  req.on('error', () => ac.abort());
});
