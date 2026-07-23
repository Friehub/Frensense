// SAFE: sanitize user input before logging
function sanitizeLogInput(input: string): string {
  return input.replace(/[\r\n\t]/g, '_').replace(/\x00/g, '');
}

app.post('/api/login', async (req, res) => {
  const { username } = req.body;
  logger.info(`Login attempt for user: ${sanitizeLogInput(username)}`);
});

app.use((err, req, res, next) => {
  logger.error(`Error: ${sanitizeLogInput(err.message)}`);
  next(err);
});
