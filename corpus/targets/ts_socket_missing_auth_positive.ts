// [frensense]
// observation: A WebSocket handler processes incoming messages without any authentication or authorization check.
// impact: Any client that can connect to the WebSocket endpoint can send arbitrary messages, potentially triggering privileged operations.
// improvement: Verify the client's authentication token on connection and validate it before processing each message.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket) => {
  socket.on('delete:user', (data: { userId: string }) => {
    deleteUser(data.userId);
  });
});
