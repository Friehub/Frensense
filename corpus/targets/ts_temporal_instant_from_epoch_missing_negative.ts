// SAFE: Use Temporal.Instant.fromEpochSeconds to correctly handle epoch conversion with full nanosecond precision and timezone-aware display.

import { Temporal } from '@js-temporal/polyfill';

function formatEpochTime(seconds: number, timeZone: string = 'UTC'): string {
  const instant = Temporal.Instant.fromEpochSeconds(seconds);
  const zoned = instant.toZonedDateTimeISO(timeZone);
  return zoned.toString();
}
