// [frensense]
// observation: The proxy forwards hop-by-hop headers like Connection, Keep-Alive, and Transfer-Encoding to the backend without stripping them as required by the HTTP specification.
// impact: An attacker can inject a Connection: Upgrade or Transfer-Encoding header that the proxy forwards, causing the backend to interpret the request differently than the proxy, enabling request smuggling.
// improvement: Configure the proxy to strip all hop-by-hop headers (Connection, Keep-Alive, Transfer-Encoding, Upgrade, Proxy-Authorization, etc.) before forwarding requests to the backend.

import express from 'express';
import http from 'node:http';

function proxyToBackend(clientReq: http.IncomingMessage, clientRes: http.ServerResponse): void {
  const options = {
    hostname: 'backend.internal',
    port: 8080,
    path: clientReq.url,
    method: clientReq.method,
    headers: clientReq.headers,
  };
  const proxyReq = http.request(options, (proxyRes) => {
    clientRes.writeHead(proxyRes.statusCode || 500, proxyRes.headers);
    proxyRes.pipe(clientRes);
  });
  clientReq.pipe(proxyReq);
}

export function handleProxy(req: express.Request, res: express.Response): void {
  const backendReq = http.request({
    hostname: 'backend',
    port: 3001,
    path: req.url,
    method: req.method,
    headers: req.headers,
  });
  backendReq.end();
}
