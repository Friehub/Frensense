// SAFE: A disconnect handler releases all per-connection resources: presence, timers, and room memberships.

import { Server } from 'socket.io';

const onlineUsers = new Map<string, string>();

const io = new Server();

io.on('connection', (socket) => {
  const userId = socket.data.userId;

  socket.join(`user:${userId}`);
  onlineUsers.set(userId, socket.id);

  const heartbeat = setInterval(() => {
    socket.emit('heartbeat', { ts: Date.now() });
  }, 30000);

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    await db.query(
      'INSERT INTO messages (roomId, senderId, text) VALUES (?, ?, ?)',
      [data.roomId, userId, data.text]
    ).run();
    io.to(data.roomId).emit('message:new', { senderId: userId, text: data.text });
  });

  socket.on('disconnect', () => {
    clearInterval(heartbeat);
    onlineUsers.delete(userId);
    for (const room of socket.rooms) {
      if (room !== socket.id) {
        socket.leave(room);
      }
    }
  });
});
