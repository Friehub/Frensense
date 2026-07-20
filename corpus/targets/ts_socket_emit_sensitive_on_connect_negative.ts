// SAFE: Only non-sensitive, minimal data is emitted on connection. PII, secrets, and bulk data are fetched on-demand through authenticated events.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.emit('session:init', {
    userId: socket.data.userId,
    displayName: socket.data.displayName,
  });

  socket.on('fetch:profile', async () => {
    const user = await db.query(
      'SELECT id, displayName, avatarUrl FROM users WHERE id = ?',
      [socket.data.userId]
    ).get();
    socket.emit('profile:data', user);
  });
});
