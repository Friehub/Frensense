// SAFE: generic error messages, full details logged server-side
import logger from './logger';

app.get('/api/files/:filename', async (req, res) => {
  try {
    const data = await fs.readFile(`./uploads/${req.params.filename}`);
    res.send(data);
  } catch (err) {
    logger.error({ err, filename: req.params.filename }, 'File read failed');
    res.status(500).json({ error: 'Failed to read file' });
  }
});

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  logger.error({ err, url: req.url }, 'Unhandled error');
  res.status(500).json({ error: 'Internal server error' });
});
