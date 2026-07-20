// SAFE: Target a specific client ID rather than broadcasting to all clients.

async function sendToClient(clientId: string, data: unknown): Promise<void> {
  const client = await self.clients.get(clientId);
  if (client) {
    client.postMessage(data);
  }
}
