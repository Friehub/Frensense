// SAFE alternative: promisified randomFill via randomBytes
import { randomBytes } from 'node:crypto';
import { createServer } from 'node:http';

async function generateToken(): Promise<string> {
  const buf = await new Promise<Buffer>((resolve, reject) => {
    randomBytes(64, (err, b) => {
      if (err) reject(err);
      else resolve(b);
    });
  });
  return buf.toString('hex');
}

const server = createServer(async (_req, res) => {
  const token = await generateToken();
  res.end(token);
});

server.listen(3000);
