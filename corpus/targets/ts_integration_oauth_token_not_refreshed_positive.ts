// [frensense]
// observation: OAuth access token used for downstream API calls without refresh logic — when the token expires, the integration silently fails or returns stale data.
// impact: Expired tokens cause silent data loss: failed syncs, missed webhooks, corrupted state. User sees stale data without any error indication.
// improvement: Implement token refresh with retry, and surface refresh failures to the user.

import { Request, Response } from 'express';

interface Integration {
  accessToken: string;
  refreshToken: string;
  provider: string;
}

const integrations = new Map<string, Integration>();

export async function callGoogleApi(userId: string): Promise<any> {
  const integration = integrations.get(userId);
  if (!integration) throw new Error('no integration');

  const resp = await fetch('https://www.googleapis.com/calendar/v3/calendars/primary/events', {
    headers: {
      Authorization: `Bearer ${integration.accessToken}`,
    },
  });
  return resp.json();
}

export async function syncCalendar(req: Request, res: Response): Promise<void> {
  const userId = req.session.userId;
  try {
    const events = await callGoogleApi(userId);
    res.json({ events });
  } catch (err) {
    res.status(502).json({ error: 'sync failed' });
  }
}
