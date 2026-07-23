// SAFE: strip hop-by-hop headers before forwarding to the backend
import express from 'express';
import http from 'node:http';

const HOP_BY_HOP = new Set([
  'connection', 'keep-alive', 'transfer-encoding', 'upgrade',
  'proxy-authorization', 'proxy-authenticate', 'te', 'trailer',
]);

function cleanHeaders(headers: http.IncomingHttpHeaders): http.OutgoingHttpHeaders {
  const cleaned: http.OutgoingHttpHeaders = {};
  for (const [key, value] of Object.entries(headers)) {
    if (!HOP_BY_HOP.has(key.toLowerCase())) {
      cleaned[key] = value;
    }
  }
  return cleaned;
}

function proxyToBackend(clientReq: http.IncomingMessage, clientRes: http.ServerResponse): void {
  const options = {
    hostname: 'backend.internal',
    port: 8080,
    path: clientReq.url,
    method: clientReq.method,
    headers: cleanHeaders(clientReq.headers),
  };
  const proxyReq = http.request(options, (proxyRes) => {
    clientRes.writeHead(proxyRes.statusCode || 500, cleanHeaders(proxyRes.headers));
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
    headers: cleanHeaders(req.headers),
  });
  backendReq.end();
}
