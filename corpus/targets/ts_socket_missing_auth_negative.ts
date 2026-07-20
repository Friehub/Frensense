// SAFE: Authentication token is validated on connection and per-message

import { Server } from 'socket.io';

const io = new Server();

io.use((socket, next) => {
  const token = socket.handshake.auth.token;
  if (!token) return next(new Error('Authentication required'));
  const user = verifyToken(token);
  if (!user) return next(new Error('Invalid token'));
  (socket as any).user = user;
  next();
});

io.on('connection', (socket) => {
  socket.on('delete:user', async (data: { userId: string }) => {
    const user = (socket as any).user;
    if (user.role !== 'admin') return;
    await deleteUser(data.userId);
  });
});
