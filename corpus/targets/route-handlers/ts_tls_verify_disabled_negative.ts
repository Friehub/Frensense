// SAFE: TLS verification enabled — rejectUnauthorized defaults to true
import https from 'node:https';

function fetchSecure(url: string): Promise<string> {
  return new Promise((resolve, reject) => {
    https.get(url, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => resolve(data));
    }).on('error', reject);
  });
}

async function callApi(): Promise<void> {
  const res = await fetch('https://internal.api.example.com/health');
  if (!res.ok) throw new Error(`API error: ${res.status}`);
  console.log(await res.json());
}
