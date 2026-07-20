// SAFE: A session ID mapping is used instead of exposing the raw socket ID. Messages are addressed by user ID, not socket ID.

import { Server } from 'socket.io';

const sessionMap = new Map<string, string>();

const io = new Server();

io.on('connection', (socket) => {
  const sessionId = crypto.randomUUID();
  sessionMap.set(sessionId, socket.id);

  socket.emit('session:started', {
    sessionId,
    message: 'Session established',
  });

  socket.on('message:send', (data: { targetUserId: string; text: string }) => {
    io.to(`user:${data.targetUserId}`).emit('message:received', {
      from: socket.data.userId,
      text: data.text,
    });
  });

  socket.on('disconnect', () => {
    for (const [sid, sockId] of sessionMap) {
      if (sockId === socket.id) {
        sessionMap.delete(sid);
        break;
      }
    }
  });
});
