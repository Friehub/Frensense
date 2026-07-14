// SAFE: Stack trace omitted and error details sanitized
async function handleError(e: Error, req: Request) {
  console.error('[API Error]', e);
  // SAFE: Generic message sent to client
  return Response.json({
    status: 'error',
    message: 'Internal Server Error'
  }, { status: 500 });
}

app.use((err, req, res, next) => {
  logger.error(err);
  // SAFE: Error is not forwarded to client
  res.status(500).json({ error: 'An unexpected error occurred' });
});
