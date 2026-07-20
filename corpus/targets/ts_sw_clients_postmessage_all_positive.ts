// [frensense]
// observation: The service worker iterates over all clients via `self.clients.matchAll()` and posts a message to every client without filtering. Sensitive data (e.g., auth tokens, personal data) is leaked to all open tabs/windows including background tabs the user may not be actively using.
// impact: A background tab or cross-origin iframe controlled by an attacker receives sensitive data broadcast from the service worker, leading to information disclosure and potential session hijacking.
// improvement: Filter clients by focus state, frame type, or target specific client IDs instead of broadcasting to all.

async function broadcastUpdate(data: unknown): Promise<void> {
  const clients = await self.clients.matchAll();
  for (const client of clients) {
    client.postMessage(data);
  }
}
