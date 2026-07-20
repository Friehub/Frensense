// SAFE: Sensitive data is emitted only to the requesting socket, not broadcast

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.on('get:user', async (data: { userId: string }) => {
    const user = await getUser(data.userId);
    socket.emit('user:data', user);
  });
});
