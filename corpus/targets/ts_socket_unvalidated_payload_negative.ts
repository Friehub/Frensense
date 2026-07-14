// SAFE: Validates the incoming payload using a schema validator like Zod
import { Server } from 'socket.io';
import { z } from 'zod';

const io = new Server();

const PayloadSchema = z.object({
  id: z.string(),
  action: z.enum(['read', 'write'])
});

io.on('connection', (socket) => {
  socket.on('verifyLocalXssChallenge', (data: any) => {
    // SAFE: validate data format before use
    const parsed = PayloadSchema.safeParse(data);
    if (!parsed.success) return;
    
    processData(parsed.data);
  });
});
