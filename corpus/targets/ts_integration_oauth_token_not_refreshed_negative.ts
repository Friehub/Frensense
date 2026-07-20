// SAFE: Token refresh on 401 response with retry.
import { Request, Response } from 'express';

interface Integration {
  accessToken: string;
  refreshToken: string;
  provider: string;
  expiresAt: number;
}

const integrations = new Map<string, Integration>();

async function refreshAccessToken(integration: Integration): Promise<string> {
  const resp = await fetch('https://oauth2.googleapis.com/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body: new URLSearchParams({
      client_id: process.env.GOOGLE_CLIENT_ID!,
      client_secret: process.env.GOOGLE_CLIENT_SECRET!,
      refresh_token: integration.refreshToken,
      grant_type: 'refresh_token',
    }),
  });
  const data = (await resp.json()) as { access_token: string; expires_in: number };
  integration.accessToken = data.access_token;
  integration.expiresAt = Date.now() + data.expires_in * 1000;
  return data.access_token;
}

export async function callGoogleApi(userId: string): Promise<any> {
  const integration = integrations.get(userId);
  if (!integration) throw new Error('no integration');

  if (Date.now() >= integration.expiresAt) {
    await refreshAccessToken(integration);
  }

  const resp = await fetch('https://www.googleapis.com/calendar/v3/calendars/primary/events', {
    headers: { Authorization: `Bearer ${integration.accessToken}` },
  });

  if (resp.status === 401) {
    await refreshAccessToken(integration);
    const retryResp = await fetch('https://www.googleapis.com/calendar/v3/calendars/primary/events', {
      headers: { Authorization: `Bearer ${integration.accessToken}` },
    });
    return retryResp.json();
  }
  return resp.json();
}

export async function syncCalendar(req: Request, res: Response): Promise<void> {
  const userId = req.session.userId;
  try {
    const events = await callGoogleApi(userId);
    res.json({ events });
  } catch (err) {
    res.status(502).json({ error: 'sync failed', detail: (err as Error).message });
  }
}
