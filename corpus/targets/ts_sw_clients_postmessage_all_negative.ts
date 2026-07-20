// SAFE: Only post messages to focused (active) clients to reduce leak surface.

async function broadcastToActiveClients(data: unknown): Promise<void> {
  const clients = await self.clients.matchAll({ type: 'window' });
  for (const client of clients) {
    if (client.focused) {
      client.postMessage(data);
    }
  }
}
