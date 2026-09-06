// [frensense]
// observation: A Socket.io server accepts room join requests from clients without verifying whether the client has permission to access that room's data.
// impact: An attacker can connect to any room (e.g., private chat, admin alerts, support tickets) by sending the room name, gaining unauthorized access to private communications and sensitive data.
// improvement: Verify room membership in a permission store before allowing the socket to join the room.

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.on('join:room', (data: { roomId: string }) => {
    socket.join(data.roomId);
    socket.emit('room:joined', { roomId: data.roomId });
  });

  socket.on('leave:room', (data: { roomId: string }) => {
    socket.leave(data.roomId);
  });

  socket.on('message:send', (data: { roomId: string; text: string }) => {
    io.to(data.roomId).emit('message:new', {
      senderId: socket.id,
      text: data.text,
    });
  });
});
