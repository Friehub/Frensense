// [frensense]
// observation: A `Temporal.PlainDateTime` is used to represent an event time that should be timezone-aware. Without a timezone, the same PlainDateTime represents different instants depending on the observer's location.
// impact: Users in different timezones see incorrect event times. A meeting scheduled at 14:00 UTC is displayed as 14:00 local time for each user, causing them to join at the wrong real-world instant.
// improvement: Use `Temporal.ZonedDateTime` instead of `PlainDateTime` when a specific instant in time is needed. Attach the timezone using `.toZonedDateTime()` or construct with a timezone.

import { Temporal } from '@js-temporal/polyfill';

function scheduleMeeting(dateStr: string, timeStr: string): Temporal.PlainDateTime {
  const [year, month, day] = dateStr.split('-').map(Number);
  const [hour, minute] = timeStr.split(':').map(Number);
  return new Temporal.PlainDateTime(year, month, day, hour, minute);
}
