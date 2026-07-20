// [frensense]
// observation: X-Content-Type-Options: nosniff header is missing, allowing browsers to MIME-sniff response content.
// impact: Browsers may interpret a response as a different content type than declared. An attacker who uploads a text file with embedded JavaScript could have it executed by a browser that MIME-sniffs it as text/html.
// improvement: Set X-Content-Type-Options: nosniff on all responses to prevent MIME-sniffing.

import express from 'express';

const app = express();

// VULNERABLE: X-Content-Type-Options not set
app.get('/api/files/:filename', (req, res) => {
  res.sendFile(`./files/${req.params.filename}`);
});

app.get('/api/download', (req, res) => {
  res.download('./data/report.csv');
});
