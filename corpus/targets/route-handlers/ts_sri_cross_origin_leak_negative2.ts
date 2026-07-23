// SAFE alternative: self-host the script instead of loading from a cross-origin CDN
import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <script src="/static/lib.min.js"></script>
    </html>
  `);
});

export function renderScriptTag(): string {
  return `<script src="/static/analytics.min.js"></script>`;
}
