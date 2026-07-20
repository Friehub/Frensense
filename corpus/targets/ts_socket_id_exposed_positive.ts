// [frensense]
// observation: The server sends the raw Socket.io socket ID to clients or uses it as a user-visible identifier without any mapping layer, making socket IDs predictable and enumerable.
// impact: Socket IDs are sequential or predictable (often incrementing by 2), enabling an attacker to enumerate connected clients, predict future socket IDs, and potentially intercept messages by guessing socket IDs in targeted emit calls.
// improvement: Never expose raw socket IDs to clients. Use session tokens, user IDs, or opaque random identifiers instead.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.emit('session:started', {
    socketId: socket.id,
    message: 'Your socket ID is ' + socket.id,
  });

  socket.on('message:send', (data: { targetSocketId: string; text: string }) => {
    io.to(data.targetSocketId).emit('message:received', {
      from: socket.id,
      text: data.text,
    });
  });
});
