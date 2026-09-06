// SAFE alternative: never embed user input inside script blocks; use JSON or DOM attributes instead
import helmet from 'helmet';
import express from 'express';

const app = express();

app.use(helmet.contentSecurityPolicy({
  directives: {
    defaultSrc: ["'self'"],
    scriptSrc: ["'self'", "'unsafe-inline'"],
  },
}));

app.get('/search', (req, res) => {
  const query = req.query.q as string;
  res.send(`
    <html>
      <div id="search-data" data-query="${query.replace(/"/g, '&quot;')}"></div>
      <script>
        var el = document.getElementById('search-data');
        var searchQuery = el.getAttribute('data-query');
        document.getElementById('results').innerHTML = searchQuery;
      </script>
    </html>
  `);
});

export function showResult(req: express.Request, res: express.Response): void {
  const term = req.query.term as string;
  res.send(`<div id="result-term" data-term="${term.replace(/"/g, '&quot;')}"></div>`);
}
