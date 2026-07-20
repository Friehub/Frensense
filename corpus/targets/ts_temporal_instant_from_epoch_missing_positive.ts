// [frensense]
// observation: The code uses `new Date(milliseconds * 1000)` to convert a Unix timestamp to a date, using the legacy Date API. This loses nanosecond precision and bypasses Temporal's robust time handling.
// impact: Microsecond/nanosecond timestamps can overflow the Date API's millisecond precision, causing incorrect epoch conversions and subtle off-by-one errors in time-sensitive systems.
// improvement: Use `Temporal.Instant.fromEpochMilliseconds()` or `Temporal.Instant.fromEpochSeconds()` for epoch conversions.

import { Temporal } from '@js-temporal/polyfill';

function formatEpochTime(seconds: number): string {
  const date = new Date(seconds * 1000);
  return date.toISOString();
}
