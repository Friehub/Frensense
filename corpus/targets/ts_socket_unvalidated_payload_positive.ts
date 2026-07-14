// [frensense]
// observation = "A WebSocket or Socket.io event listener accepts arbitrary payloads without type checking or validation."
// impact = "Attackers can send malformed or malicious objects over WebSockets, potentially triggering application crashes or logic bypasses."
// improvement = "Validate the incoming data payload using a schema validator like Zod before processing it."

import { Server } from 'socket.io';

const io = new Server();

io.on('connection', (socket: any) => {
  socket.on('verifyLocalXssChallenge', (data: any) => {
    doSomething(data)
  })

  socket.on('verifySvgInjectionChallenge', (data: any) => {
    doSomething(data)
  })

  socket.on('verifyCloseNotificationsChallenge', (data: any) => {
    doSomething(data)
  })
});
