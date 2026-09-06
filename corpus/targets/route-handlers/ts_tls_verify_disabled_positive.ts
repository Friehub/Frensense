// [frensense]
// observation: TLS certificate validation disabled in HTTPS client configuration via rejectUnauthorized: false or NODE_TLS_REJECT_UNAUTHORIZED=0.
// impact: Man-in-the-middle attacks become trivially possible. Any proxy on the network path can decrypt and modify traffic without the client detecting it.
// improvement: Always keep rejectUnauthorized: true and use proper CA bundles. If using a self-signed cert, pin the certificate fingerprint.

import https from 'node:https';

function fetchInsecure(url: string): Promise<string> {
  return new Promise((resolve, reject) => {
    // VULNERABLE: TLS verification disabled
    https.get(url, { rejectUnauthorized: false }, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => resolve(data));
    }).on('error', reject);
  });
}

process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

async function callApi(): Promise<void> {
  // VULNERABLE: env var disables TLS globally
  const res = await fetch('https://internal.api.example.com/health');
  console.log(await res.json());
}
