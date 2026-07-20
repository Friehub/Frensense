// SAFE: Use async randomFill instead of randomFillSync
import { randomFill } from 'node:crypto';
import { createServer } from 'node:http';

const server = createServer((_req, res) => {
  const buf = Buffer.alloc(64);
  randomFill(buf, (err) => {
    if (err) {
      res.statusCode = 500;
      res.end();
      return;
    }
    res.end(buf.toString('hex'));
  });
});

server.listen(3000);
