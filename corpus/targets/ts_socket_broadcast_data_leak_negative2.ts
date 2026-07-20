// SAFE: Data is emitted to a specific room that the user has joined after authentication

import { Server } from 'socket.io';

const io = new Server();

io.use((socket, next) => {
  const token = socket.handshake.auth.token;
  const user = verifyToken(token);
  if (!user) return next(new Error('Unauthorized'));
  (socket as any).userId = user.id;
  next();
});

io.on('connection', (socket) => {
  const userId = (socket as any).userId;
  socket.join(`user:${userId}`);

  socket.on('get:user', async () => {
    const user = await getUser(userId);
    io.to(`user:${userId}`).emit('user:data', user);
  });
});
