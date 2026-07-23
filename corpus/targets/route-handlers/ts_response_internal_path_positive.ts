// [frensense]
// observation: File system paths, internal IP addresses, or architecture details included in API error responses.
// impact: Attackers learn the exact directory structure (/var/www/app/dist/), OS type, or internal network topology. This enables path-specific exploits and targeted directory traversal attacks.
// improvement: Map internal errors to generic messages. Log the full error server-side and return a sanitized message to the client.

app.get('/api/files/:filename', async (req, res) => {
  try {
    const data = await fs.readFile(`./uploads/${req.params.filename}`);
    res.send(data);
  } catch (err) {
    // VULNERABLE: leaks internal path
    res.status(500).json({ error: `Failed to read file: /var/www/uploads/${req.params.filename}` });
  }
});

app.use((err: Error, req: Request, res: Response, next: NextFunction) => {
  // VULNERABLE: full path in stack
  res.status(500).json({
    error: err.message,
    path: __filename,
    cwd: process.cwd(),
  });
});
