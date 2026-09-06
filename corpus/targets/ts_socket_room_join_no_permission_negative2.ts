// SAFE: Room membership is checked via middleware before join, and a permission cache reduces DB load.

import { Server } from 'socket.io';

const roomPermissions = new Map<string, Set<string>>();

async function loadRoomPermissions(roomId: string): Promise<void> {
  if (roomPermissions.has(roomId)) return;
  const rows = await db.query('SELECT userId FROM room_members WHERE roomId = ?', [roomId]).all();
  roomPermissions.set(roomId, new Set(rows.map((r: any) => r.userId)));
}

async function canAccessRoom(userId: string, roomId: string): Promise<boolean> {
  await loadRoomPermissions(roomId);
  return roomPermissions.get(roomId)?.has(userId) ?? false;
}

const io = new Server();

io.on('connection', (socket) => {
  const userId = socket.data.userId;

  socket.on('join:room', async (data: { roomId: string }) => {
    if (!(await canAccessRoom(userId, data.roomId))) {
      socket.emit('error', { message: 'Access denied' });
      return;
    }
    socket.join(data.roomId);
  });

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    if (!(await canAccessRoom(userId, data.roomId))) return;
    socket.to(data.roomId).emit('message:new', {
      senderId: userId,
      text: data.text,
    });
  });
});
