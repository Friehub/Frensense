// SAFE: API key is never included in client-facing error responses
import { Router, Request, Response } from 'express';

const router = Router();
const API_KEY = process.env.API_KEY!;

export async function proxyRequest(req: Request, res: Response): Promise<void> {
  try {
    const result = await fetch('https://upstream.example.com/data', {
      headers: { Authorization: `Bearer ${API_KEY}` },
    });
    const data = await result.json();
    res.json(data);
  } catch (err) {
    console.error('Upstream proxy failed:', err.message);
    res.status(502).json({ error: 'Upstream service unavailable' });
  }
}
