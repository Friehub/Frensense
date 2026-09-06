// SAFE: Room join is gated by a membership check against the database before allowing socket.join.

import { Server } from 'socket.io';

interface AuthSocket {
  userId: string;
}

async function isRoomMember(userId: string, roomId: string): Promise<boolean> {
  const membership = await db.query(
    'SELECT 1 FROM room_members WHERE userId = ? AND roomId = ?',
    [userId, roomId]
  ).get();
  return !!membership;
}

const io = new Server();

io.on('connection', (socket) => {
  const auth = socket.data as AuthSocket;

  socket.on('join:room', async (data: { roomId: string }) => {
    const allowed = await isRoomMember(auth.userId, data.roomId);
    if (!allowed) {
      socket.emit('error', { message: 'Not a member of this room' });
      return;
    }
    socket.join(data.roomId);
    socket.emit('room:joined', { roomId: data.roomId });
  });

  socket.on('leave:room', (data: { roomId: string }) => {
    socket.leave(data.roomId);
  });

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    const allowed = await isRoomMember(auth.userId, data.roomId);
    if (!allowed) return;
    io.to(data.roomId).emit('message:new', {
      senderId: auth.userId,
      text: data.text,
    });
  });
});
