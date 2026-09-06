// SAFE alternative: structured logging with extra field, not interpolated
app.post('/api/login', async (req, res) => {
  const { username } = req.body;
  // SAFE: user input as structured field, not interpolated into message
  logger.info('Login attempt', { username, ip: req.ip });
});

app.use((err, req, res, next) => {
  logger.error('Request error', { error: err.message, url: req.url });
  next(err);
});
