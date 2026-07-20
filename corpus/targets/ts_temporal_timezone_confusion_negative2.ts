// SAFE: Convert PlainDateTime to an Instant via a timezone before serializing.

import { Temporal } from '@js-temporal/polyfill';

function scheduleEvent(dateStr: string, timeStr: string, timeZone: string): Temporal.Instant {
  const [year, month, day] = dateStr.split('-').map(Number);
  const [hour, minute] = timeStr.split(':').map(Number);
  const plain = new Temporal.PlainDateTime(year, month, day, hour, minute);
  const zoned = plain.toZonedDateTime(timeZone);
  return zoned.toInstant();
}
