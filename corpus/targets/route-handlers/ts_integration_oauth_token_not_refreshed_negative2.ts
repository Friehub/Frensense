// SAFE: Background token refresh via dedicated refresh agent.
import { Request, Response } from 'express';

interface Integration {
  accessToken: string;
  refreshToken: string;
  provider: string;
}

const integrations = new Map<string, Integration>();

async function ensureValidToken(integration: Integration): Promise<void> {
  const resp = await fetch('https://www.googleapis.com/calendar/v3/users/me/calendarList', {
    headers: { Authorization: `Bearer ${integration.accessToken}` },
  });
  if (resp.status === 401) {
    const tokenResp = await fetch('https://oauth2.googleapis.com/token', {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: new URLSearchParams({
        client_id: process.env.GOOGLE_CLIENT_ID!,
        client_secret: process.env.GOOGLE_CLIENT_SECRET!,
        refresh_token: integration.refreshToken,
        grant_type: 'refresh_token',
      }),
    });
    const data = (await tokenResp.json()) as { access_token: string };
    integration.accessToken = data.access_token;
  }
}

export async function syncCalendar(req: Request, res: Response): Promise<void> {
  const userId = req.session.userId;
  const integration = integrations.get(userId);
  if (!integration) {
    res.status(400).json({ error: 'no integration configured' });
    return;
  }

  await ensureValidToken(integration);

  const eventsResp = await fetch('https://www.googleapis.com/calendar/v3/calendars/primary/events', {
    headers: { Authorization: `Bearer ${integration.accessToken}` },
  });
  if (!eventsResp.ok) {
    res.status(502).json({ error: 'sync failed', detail: eventsResp.statusText });
    return;
  }
  const events = await eventsResp.json();
  res.json({ events });
}
