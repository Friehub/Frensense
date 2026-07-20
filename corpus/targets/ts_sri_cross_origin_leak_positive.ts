// [frensense]
// observation: A script is loaded with an integrity attribute but without the crossorigin="anonymous" attribute. When CORS is not explicitly requested via the crossorigin attribute, the browser cannot perform the integrity check because the script response may not be CORS-enabled.
// impact: Modern browsers skip the SRI integrity check on cross-origin scripts that do not include the crossorigin attribute. The script loads and executes even if its content does not match the integrity hash, rendering the SRI protection useless.
// improvement: Always add crossorigin="anonymous" to cross-origin <script> elements that include an integrity attribute.

import express from 'express';

const app = express();

app.get('/app', (req, res) => {
  res.send(`
    <html>
      <script src="https://cdn.example.com/lib.js"
              integrity="sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC"></script>
    </html>
  `);
});

export function renderScriptTag(): string {
  return `<script src="https://cdn.example.com/analytics.js"
                  integrity="sha384-abc123def456"></script>`;
}
