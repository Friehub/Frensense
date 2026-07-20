// SAFE: add crossorigin="anonymous" alongside integrity for cross-origin scripts
import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <script src="https://cdn.example.com/lib.js"
              integrity="sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC"
              crossorigin="anonymous"></script>
    </html>
  `);
});

export function renderScriptTag(): string {
  return `<script src="https://cdn.example.com/analytics.js"
                  integrity="sha384-abc123def456"
                  crossorigin="anonymous"></script>`;
}
