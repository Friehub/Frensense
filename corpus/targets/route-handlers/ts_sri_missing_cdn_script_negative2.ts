// SAFE alternative: self-host third-party libraries instead of loading from CDN
import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <head>
        <script src="/static/vendor/react.production.min.js"></script>
        <script src="/static/vendor/react-dom.production.min.js"></script>
      </head>
      <body>
        <div id="root"></div>
      </body>
    </html>
  `);
});

export function renderAppShell(): string {
  return `
    <script src="/static/vendor/jquery.min.js"></script>
    <script src="/static/app.js"></script>
  `;
}
