// SAFE alternative: use a dedicated reverse proxy library that handles hop-by-hop stripping automatically
import express from 'express';
import httpProxy from 'http-proxy';

const proxy = httpProxy.createProxyServer({
  target: 'http://backend.internal:8080',
});

proxy.on('proxyReq', (proxyReq, req, res, options) => {
  proxyReq.setHeader('X-Forwarded-For', req.socket.remoteAddress || '');
  proxyReq.removeHeader('transfer-encoding');
  proxyReq.removeHeader('connection');
  proxyReq.removeHeader('keep-alive');
  proxyReq.removeHeader('upgrade');
});

export function setupProxy(app: express.Application): void {
  app.all('/api/*', (req, res) => {
    proxy.web(req, res);
  });
}
