// [frensense]
// observation: Socket.io handlers allocate resources (room subscriptions, event listeners, timers, presence state) on connection but never clean them up on disconnect, causing resources to accumulate over time.
// impact: Socket disconnects leave behind stale room memberships, zombie timers, and accumulated presence state, leading to memory leaks, incorrect room member counts, and notifications sent to disconnected users.
// improvement: Register a disconnect handler that releases all resources held by the socket, including removing from rooms, clearing timers, and updating presence state.

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
});
