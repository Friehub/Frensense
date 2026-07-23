// SAFE: Uses structured logging with Pino and redact option
import pino from 'pino';

const logger = pino({
  redact: ['req.headers.authorization', 'req.headers["x-api-key"]', 'req.body.password', 'req.body.token']
});

export async function handleRequest(req: Request): Promise<Response> {
  logger.info({ req }, 'Incoming request');
  const res = await fetch('https://api.example.com/data', {
    headers: { Authorization: `Bearer ${process.env.API_KEY}` }
  });
  return res;
}
