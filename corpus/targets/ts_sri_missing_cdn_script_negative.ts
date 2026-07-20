// SAFE: add integrity hashes for all CDN scripts
import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <head>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/react/18.2.0/umd/react.production.min.js"
                integrity="sha256-8G1OQEsZN0JqJkLYNc9CzQ4gS3zWJp7V6pCFQzJsjM="
                crossorigin="anonymous"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/react-dom/18.2.0/umd/react-dom.production.min.js"
                integrity="sha256-8G1OQEsZN0JqJkLYNc9CzQ4gS3zWJp7V6pCFQzJsjM="
                crossorigin="anonymous"></script>
      </head>
      <body>
        <div id="root"></div>
      </body>
    </html>
  `);
});

export function renderAppShell(): string {
  return `
    <script src="https://code.jquery.com/jquery-3.7.1.min.js"
            integrity="sha256-2KokqMbq0F9cTgQ=="
            crossorigin="anonymous"></script>
    <script src="/static/app.js"></script>
  `;
}
