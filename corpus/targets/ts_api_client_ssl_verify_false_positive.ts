// [frensense]
// observation: SSL/TLS certificate verification is disabled for outbound API requests, often using NODE_TLS_REJECT_UNAUTHORIZED=0 or setting rejectUnauthorized to false.
// impact: The client accepts any TLS certificate, including self-signed or伪造 certificates, enabling man-in-the-middle attacks. An attacker on the network path can intercept, read, and modify all data sent to or received from the API.
// improvement: Always enable certificate verification. Use proper CA-signed certificates or add custom CA certificates to the trust store.

import https from 'https';

async function fetchData() {
  const agent = new https.Agent({ rejectUnauthorized: false });
  const response = await fetch('https://internal-api.example.com/data', {
    agent,
  });
  return response.json();
}
