// [frensense]
// observation: Socket.io emits sensitive data using io.emit() or broadcast, sending it to all connected clients.
// impact: Sensitive user data is sent to unintended recipients, including other users connected to the same server.
// improvement: Use targeted emission (socket.emit() or to(room)) instead of global broadcast for sensitive data.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.on('get:user', async (data: { userId: string }) => {
    const user = await getUser(data.userId);
    io.emit('user:data', user);
  });
});
