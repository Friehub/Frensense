// SAFE: Use Temporal.Instant.fromEpochMilliseconds when working with millisecond-precision timestamps.

import { Temporal } from '@js-temporal/polyfill';

function formatEpochTimeMs(milliseconds: number, locale: string = 'en-US'): string {
  const instant = Temporal.Instant.fromEpochMilliseconds(milliseconds);
  return instant.toLocaleString(locale);
}
