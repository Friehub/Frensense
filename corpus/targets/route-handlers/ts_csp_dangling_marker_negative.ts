// SAFE: escape user input before embedding in script context
import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", "'unsafe-inline'"],
  },
}));

function escapeJsString(s: string): string {
  return s.replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\n/g, '\\n').replace(/\r/g, '\\r');
}

app.get('/search', (req, res) => {
  const query = req.query.q as string;
  const safeQuery = escapeJsString(query);
  res.send(`
    <html>
      <script>
        var searchQuery = "${safeQuery}";
        document.getElementById('results').innerHTML = searchQuery;
      </script>
    </html>
  `);
});

export function showResult(req: express.Request, res: express.Response): void {
  const term = escapeJsString(req.query.term as string);
  res.send(`<script>var term = "${term}";</script>`);
}
