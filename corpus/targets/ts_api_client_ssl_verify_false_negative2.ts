// SAFE: custom CA certificate is added to the trust store explicitly

import https from 'https';
import fs from 'fs';

function createVerifiedAgent() {
  return new https.Agent({
    ca: fs.readFileSync('/etc/ssl/certs/ca-certificates.crt'),
    rejectUnauthorized: true,
  });
}

async function fetchData() {
  const agent = createVerifiedAgent();
  const response = await fetch('https://internal-api.example.com/data', { agent });
  return response.json();
}
