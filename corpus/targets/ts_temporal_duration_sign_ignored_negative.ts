// SAFE: Pass the duration directly — Temporal correctly handles negative durations.

import { Temporal } from '@js-temporal/polyfill';

function applyOffset(instant: Temporal.Instant, durationStr: string): Temporal.Instant {
  const duration = Temporal.Duration.from(durationStr);
  return instant.add(duration);
}
