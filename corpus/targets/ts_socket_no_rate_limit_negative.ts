// SAFE: A token-bucket rate limiter is applied per socket on message:send events, preventing event floods.

import { Server } from 'socket.io';

class TokenBucket {
  private tokens: number;
  private lastRefill: number;

  constructor(private maxTokens: number, private refillRate: number) {
    this.tokens = maxTokens;
    this.lastRefill = Date.now();
  }

  tryConsume(count: number): boolean {
    this.refill();
    if (this.tokens >= count) {
      this.tokens -= count;
      return true;
    }
    return false;
  }

  private refill() {
    const now = Date.now();
    const elapsed = (now - this.lastRefill) / 1000;
    this.tokens = Math.min(this.maxTokens, this.tokens + elapsed * this.refillRate);
    this.lastRefill = now;
  }
}

const io = new Server();

io.on('connection', (socket) => {
  const bucket = new TokenBucket(10, 1);

  socket.on('message:send', async (data: { roomId: string; text: string }) => {
    if (!bucket.tryConsume(1)) {
      socket.emit('error', { message: 'Rate limit exceeded. Slow down.' });
      return;
    }
    await db.query(
      'INSERT INTO messages (roomId, senderId, text) VALUES (?, ?, ?)',
      [data.roomId, socket.data.userId, data.text]
    ).run();
    io.to(data.roomId).emit('message:new', {
      senderId: socket.data.userId,
      text: data.text,
    });
  });

  socket.on('typing:start', (data: { roomId: string }) => {
    socket.to(data.roomId).emit('typing:update', {
      userId: socket.data.userId,
      typing: true,
    });
  });
});
