// SAFE: Initial connection emits only a session identifier. All data fetches require explicit client requests with server-side authorization.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.emit('session:ready', { sessionId: socket.data.sessionId });

  socket.on('get:recent-orders', async () => {
    const orders = await db.query(
      'SELECT id, total, status, createdAt FROM orders WHERE userId = ? ORDER BY createdAt DESC LIMIT 10',
      [socket.data.userId]
    ).all();
    socket.emit('orders:list', orders);
  });

  socket.on('get:notifications', async () => {
    const notifications = await db.query(
      'SELECT id, type, message, read FROM notifications WHERE userId = ? AND read = 0',
      [socket.data.userId]
    ).all();
    socket.emit('notifications:list', notifications);
  });
});
