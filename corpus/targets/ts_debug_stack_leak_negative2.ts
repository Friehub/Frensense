// SAFE: Strips stack trace and error details before sending response
async function handleError(e: Error, req: Request) {
  const sanitized = { status: "error", message: "Internal Server Error" };
  return Response.json(sanitized, { status: 500 });
}

app.use((err, req, res, next) => {
  logger.error(err, { path: req.path });
  res.status(500).json({ error: "Internal Server Error" });
});
