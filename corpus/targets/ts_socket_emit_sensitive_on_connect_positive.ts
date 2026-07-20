// [frensense]
// observation: Upon a new WebSocket connection, the server immediately emits sensitive user data (e.g., full user profile, PII, access tokens) to the connecting client without verifying what data the client is authorized to receive.
// impact: Sensitive data is sent to the client on every reconnection, and if the connection is intercepted or the client is malicious, the data is exposed. This also over-fetches data the client may not need.
// improvement: Only emit the minimal data required for the initial connection state, and never include secrets, tokens, or PII in the initial handshake payload.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', async (socket) => {
  const user = await db.query('SELECT * FROM users WHERE id = ?', [socket.data.userId]).get();

  socket.emit('session:init', {
    userId: user.id,
    email: user.email,
    displayName: user.displayName,
    ssn: user.ssn,
    role: user.role,
    apiKey: user.apiKey,
    sessionToken: user.sessionToken,
    recentOrders: await db.query('SELECT * FROM orders WHERE userId = ?', [user.id]).all(),
  });
});
