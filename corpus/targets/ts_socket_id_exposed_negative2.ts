// SAFE: Communication uses opaque channel tokens instead of socket IDs, preventing enumeration.

import { Server } from 'socket.io';

const activeChannels = new Map<string, { socketId: string; userId: string }>();

const io = new Server();

io.on('connection', (socket) => {
  const channelToken = crypto.randomUUID();
  activeChannels.set(channelToken, { socketId: socket.id, userId: socket.data.userId });

  socket.emit('channel:assigned', { channelToken });

  socket.on('message:send', (data: { channelToken: string; text: string }) => {
    const target = activeChannels.get(data.channelToken);
    if (!target) {
      socket.emit('error', { message: 'Invalid channel token' });
      return;
    }
    io.to(target.socketId).emit('message:received', {
      from: socket.data.userId,
      text: data.text,
    });
  });

  socket.on('disconnect', () => {
    for (const [token, info] of activeChannels) {
      if (info.socketId === socket.id) {
        activeChannels.delete(token);
        break;
      }
    }
  });
});
