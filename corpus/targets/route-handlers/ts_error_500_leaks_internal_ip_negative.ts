// SAFE: Generic error page without any system metadata.
import { Request, Response, NextFunction } from 'express';

export function errorHandler(err: Error, req: Request, res: Response, _next: NextFunction): void {
  console.error('Unhandled error:', err.message, err.stack);
  res.status(500).send(`
    <html>
      <body>
        <h1>Internal Server Error</h1>
        <p>Something went wrong on our end.</p>
        <p>Please try again later or contact support with your request ID.</p>
      </body>
    </html>
  `);
}
