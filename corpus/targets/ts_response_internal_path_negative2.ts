// SAFE alternative: custom error class with user-safe message
class AppError extends Error {
  constructor(public userMessage: string, public internalMessage: string, public status: number = 500) {
    super(internalMessage);
  }
}

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  if (err instanceof AppError) {
    logger.error({ err, url: req.url }, err.internalMessage);
    return res.status(err.status).json({ error: err.userMessage });
  }
  logger.error({ err, url: req.url }, 'Unhandled error');
  res.status(500).json({ error: 'An unexpected error occurred' });
});
