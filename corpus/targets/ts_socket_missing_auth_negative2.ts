// SAFE: Each message type is gated by a permission check against the user's role

import { Server } from 'socket.io';

const io = new Server();

const PERMISSIONS: Record<string, string[]> = {
  'delete:user': ['admin'],
  'update:profile': ['user', 'admin'],
  'read:analytics': ['admin', 'analyst']
};

io.use((socket, next) => {
  const token = socket.handshake.auth.token;
  const user = verifyToken(token);
  if (!user) return next(new Error('Unauthorized'));
  (socket as any).user = user;
  next();
});

io.on('connection', (socket) => {
  for (const [event, allowedRoles] of Object.entries(PERMISSIONS)) {
    socket.on(event, async (data: any, callback?: Function) => {
      const user = (socket as any).user;
      if (!allowedRoles.includes(user.role)) {
        callback?.({ error: 'Forbidden' });
        return;
      }
      await handleEvent(event, data);
      callback?.({ ok: true });
    });
  }
});
