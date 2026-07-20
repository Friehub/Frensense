// SAFE: Disconnect cleanup uses a resource tracker that automatically releases all registered resources.

import { Server } from 'socket.io';

class ConnectionResources {
  private timers: ReturnType<typeof setInterval>[] = [];
  private rooms: string[] = [];

  addTimer(timer: ReturnType<typeof setInterval>) {
    this.timers.push(timer);
  }

  addRoom(room: string) {
    this.rooms.push(room);
  }

  release(socket: any) {
    for (const timer of this.timers) clearInterval(timer);
    for (const room of this.rooms) socket.leave(room);
    this.timers = [];
    this.rooms = [];
  }
}

const onlineUsers = new Map<string, string>();

const io = new Server();

io.on('connection', (socket) => {
  const userId = socket.data.userId;
  const resources = new ConnectionResources();

  socket.join(`user:${userId}`);
  resources.addRoom(`user:${userId}`);
  onlineUsers.set(userId, socket.id);

  const heartbeat = setInterval(() => {
    socket.emit('heartbeat', { ts: Date.now() });
  }, 30000);
  resources.addTimer(heartbeat);

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    await db.query(
      'INSERT INTO messages (roomId, senderId, text) VALUES (?, ?, ?)',
      [data.roomId, userId, data.text]
    ).run();
    io.to(data.roomId).emit('message:new', { senderId: userId, text: data.text });
  });

  socket.on('disconnect', () => {
    resources.release(socket);
    onlineUsers.delete(userId);
  });
});
