// SAFE: Use ZonedDateTime so the meeting time is pinned to a specific timezone.

import { Temporal } from '@js-temporal/polyfill';

function scheduleMeeting(dateStr: string, timeStr: string, timeZone: string = 'America/New_York'): Temporal.ZonedDateTime {
  const [year, month, day] = dateStr.split('-').map(Number);
  const [hour, minute] = timeStr.split(':').map(Number);
  const plain = new Temporal.PlainDateTime(year, month, day, hour, minute);
  return plain.toZonedDateTime(timeZone);
}
