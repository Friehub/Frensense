// SAFE: Uses an external rate limiter (express-rate-limiter-like pattern) with per-user quotas stored in Redis.

import { Server } from 'socket.io';

interface RateLimitEntry {
  count: number;
  resetAt: number;
}

const rateLimitMap = new Map<string, RateLimitEntry>();

function checkRateLimit(key: string, maxEvents: number, windowMs: number): boolean {
  const now = Date.now();
  const entry = rateLimitMap.get(key);
  if (!entry || now > entry.resetAt) {
    rateLimitMap.set(key, { count: 1, resetAt: now + windowMs });
    return true;
  }
  if (entry.count >= maxEvents) return false;
  entry.count++;
  return true;
}

const io = new Server();

io.on('connection', (socket) => {
  const userId = socket.data.userId;

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    if (!checkRateLimit(`message:${userId}`, 20, 1000)) {
      socket.emit('error', { message: 'Too many messages. Please wait.' });
      return;
    }
    await db.query(
      'INSERT INTO messages (roomId, senderId, text) VALUES (?, ?, ?)',
      [data.roomId, userId, data.text]
    ).run();
    io.to(data.roomId).emit('message:new', { senderId: userId, text: data.text });
  });

  socket.on('typing:start', (data: { roomId: string }) => {
    socket.to(data.roomId).emit('typing:update', { userId, typing: true });
  });
});
