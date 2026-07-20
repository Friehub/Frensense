// SAFE: Incoming WebSocket data is validated with a robust validation library

import { Server } from 'socket.io';
import { z } from 'zod';

const io = new Server();

const DeleteUserSchema = z.object({
  userId: z.string().uuid(),
  reason: z.string().max(200).optional()
});

io.on('connection', (socket) => {
  socket.on('delete:user', (data: unknown) => {
    const parsed = DeleteUserSchema.safeParse(data);
    if (!parsed.success) {
      socket.emit('error', { message: 'Invalid payload' });
      return;
    }
    deleteUser(parsed.data.userId);
  });
});
