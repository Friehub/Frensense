// [frensense]
// observation: Socket.io event handlers process incoming messages without any rate limiting, allowing a single client to flood the server with events.
// impact: An attacker can send thousands of events per second, causing excessive database writes, broadcasting spam to other users, CPU exhaustion, and denial of service.
// improvement: Apply per-socket or per-user rate limiting to all event handlers that trigger side effects or expensive operations.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    await db.query(
      'INSERT INTO messages (roomId, senderId, text) VALUES (?, ?, ?)',
      [data.roomId, socket.data.userId, data.text]
    ).run();
    io.to(data.roomId).emit('message:new', {
      senderId: socket.data.userId,
      text: data.text,
    });
  });

  socket.on('typing:start', (data: { roomId: string }) => {
    socket.to(data.roomId).emit('typing:update', {
      userId: socket.data.userId,
      typing: true,
    });
  });
});
